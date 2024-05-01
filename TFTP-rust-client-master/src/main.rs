use std::cmp::min;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{Receiver, Sender};
use crate::encdec::Opcode;
use std::env;

mod encdec;
mod connection_handler;

struct State {
    terminate: RwLock<bool>,
    disc: RwLock<bool>,
    dirq: RwLock<bool>,
    await_ack: RwLock<u16>,
    connected: RwLock<bool>,
    filename: RwLock<String>,
}


fn handle_ack(bytes: Vec<u8>, state: Arc<State>, tx: &Sender<i32>, rx: &Receiver<i32>) -> Option<Vec<u8>> {
    println!("Handling ACK");
    if bytes.len() != 4 {
        println!("Invalid ACK packet");
        return None;
    }
    let block = (bytes[2] as u16) << 8 | bytes[3] as u16;
    if *state.disc.read().unwrap() && block == 0 {
        *state.terminate.write().unwrap() = true;
        *state.disc.write().unwrap() = false;
    }
     let ack = {*state.await_ack.read().unwrap()};
     if block == ack {
         tx.send(ack as i32).unwrap();
     }
    None
}

fn handle_data(bytes: Vec<u8>, state: Arc<State>, tx: &Sender<i32>, rx: &Receiver<i32>) -> Option<Vec<u8>> {
    println!("Handling DATA");

    let size = (bytes[2] as u16) << 8 | bytes[3] as u16;
    let block_num = (bytes[4] as u16) << 8 | bytes[5] as u16;
    let awk = {*state.await_ack.read().unwrap()};
    let dirq_state = {*state.dirq.read().unwrap()};
    if awk == block_num {
        if !dirq_state {
            let filename = { state.filename.read().unwrap().clone() };
            let mut file = OpenOptions::new().append(true).open(filename);
            let ret = match file {
                Ok(mut f) => {
                    f.write(bytes[6..].as_ref()).unwrap();
                    if bytes.len() < 518 {
                        { *state.await_ack.write().unwrap() = 0};
                        tx.send(0).unwrap();
                    } else { *state.await_ack.write().unwrap() = block_num + 1 };
                    let bytes = vec![0x0u8, 0x4u8, (block_num >> 8) as u8, (block_num & 0xFF) as u8];
                    Some(bytes)
                }
                Err(e) => {
                    tx.send(-1).unwrap();
                    println!("Error opening file: {}", e);
                    None
                }
            };
            return ret;
        } else {
            let bytes_split = bytes[6..].split(|&x| x == 0x0);
            let strings = bytes_split.map(|x| String::from_utf8_lossy(x));
            for s in strings {
                println!("{}", s);
            }
            if bytes.len() < 518 {
                { *state.await_ack.write().unwrap() = 0 };
                {*state.dirq.write().unwrap() = false };
                tx.send(0).unwrap();
            } else { *state.await_ack.write().unwrap() = block_num + 1 };
            let bytes = vec![0x0u8, 0x4u8, (block_num >> 8) as u8, (block_num & 0xFF) as u8];
            return Some(bytes);
        }
    }
    tx.send(block_num as i32).unwrap();
    println!("Wrong block number received for data");
    None
}


fn check_if_command_is_valid(command: &str, split: Vec<&str>) -> bool {
    match command {
        "LOGRQ" | "RRQ" | "WRQ" | "DELRQ" => {
            if split.len() < 2 {
                println!("Invalid command");
                return false;
            } else {
                let username = split[1..].join(" ");
                if username.len() > 255 || username.len() == 0 {
                    println!("Invalid filename or username length");
                    return false;
                }
            }
        }
        "DIRQ" | "DISC" => {
            if split.len() != 1 {
                println!("Invalid command");
                return false;
            }
        }
        &_ => {
            println!("Invalid command");
            return false;
        }
    }
    true
}

fn handle_bcast(bytes: Vec<u8>, state: Arc<State>, tx: &Sender<i32>, rx: &Receiver<i32>) -> Option<Vec<u8>> {
    println!("Handling BCAST");
    let adddel = bytes[2];
    let filename = String::from_utf8_lossy(&bytes[3..]);
    match adddel {
        0x0 => {
            println!("BCAST del {}", filename);
        }
        0x1 => {
            println!("BCAST add: {}", filename);
        }
        _ => {
            println!("Invalid BCAST");
        }
    }
    None
}

fn handle_error(bytes: Vec<u8>, state: Arc<State>, tx: &Sender<i32>, rx: &Receiver<i32>) -> Option<Vec<u8>> {
    println!("Handling ERROR");
    let err_code = (bytes[2] as u16) << 8 | bytes[3] as u16;
    if bytes.len() > 4 {
        let err_msg = String::from_utf8_lossy(&bytes[4..]);
        println!("Error code: {}", err_code);
        println!("Error message: {}", err_msg);
    }
    tx.send(err_code as i32).unwrap();
    None
}
fn listen(opcode: Opcode, bytes: Vec<u8>, state: Arc<State>, tx: &Sender<i32>, rx: &Receiver<i32>, connection_handler: &Arc<Mutex<connection_handler::ConnectionHandler>>) {
        let res = match opcode {
            Opcode::ACK =>  handle_ack(bytes, state, tx, rx),
            Opcode::DATA => handle_data(bytes, state, tx, rx),
            Opcode::BCAST => handle_bcast(bytes, state, tx, rx),
            Opcode::ERROR => handle_error(bytes, state, tx, rx),
            _ => {println!("Invalid opcode encountered {:?}, this opcode", opcode); None}
        };
        if let Some(bytes) = res {
            {connection_handler.lock().unwrap().send(bytes.clone())};
        }
    }


fn handle_command(command: &str, args: &String, state: &Arc<State>, tx: &Sender<i32>, rx: &Receiver<i32>,
                  connection_handler: &Arc<Mutex<connection_handler::ConnectionHandler>>) {
    match command {
        "LOGRQ" => {
            let mut bytes = vec![0x0u8, 0x7u8];
            let username = args.clone();
            bytes.extend(username.into_bytes());
            if let Ok(mut awack) = state.await_ack.write(){
                *awack = 0;
            }
            {connection_handler.lock().unwrap().send(bytes);}
            match rx.recv() {
                Ok(0) => {
                    *state.connected.write().unwrap() = true;
                }
                Ok(_) => {

                }
                Err(_) => {
                    panic!("rx is poisoned")
                }
            }

        }
        "RRQ" => {
            let mut bytes= vec![0x0u8, 0x1u8];
            let filename = args.clone();
            {*state.filename.write().unwrap() = filename.clone()};
            bytes.extend(filename.clone().into_bytes());
            // create a file
            let f = File::create(&filename);
            match f {
                Ok(_) => {
                    println!("File created");
                    {connection_handler.lock().unwrap().send(bytes)};
                }
                Err(e) => {
                    println!("Error creating file: {}", e);
                }
            }
            {*state.await_ack.write().unwrap() = 1};
            match rx.recv() {
                Ok(0) => {println!("RRQ {} complete", filename)}
                Ok(_) => {
                    println!("RRQ failed deleting file");
                    //delete file
                    let f = File::open(&filename);
                    match f {
                        Ok(_) => {
                            std::fs::remove_file(&filename).unwrap();
                        }
                        Err(e) => {
                            println!("Error deleting file: {}", e);
                        }
                    }

                }
                Err(_) => {
                    //delete file
                    let f = File::open(&filename);
                    match f {
                        Ok(_) => {
                            std::fs::remove_file(&filename).unwrap();
                            println!("File deleted RRQ failed");
                        }
                        Err(e) => {
                            println!("Error deleting file: {}", e);
                        }
                    }
                }
            }

        }
        "WRQ" => {
            let mut bytes= vec![0x0u8, 0x2u8];
            let filename = args.clone();
            {*state.filename.write().unwrap() = filename.clone()};
            bytes.extend(filename.clone().into_bytes());
            let f = File::open(&filename);
            match f {
                Ok(_) => {
                    //set writing queue
                    let mut writing_queue = vec![];
                    let mut buffer:Vec<u8> = vec![];
                    f.unwrap().read_to_end(&mut buffer).unwrap();
                    let number_of_blocks = buffer.len() / 512 + 1;
                    for i in 0..number_of_blocks {
                        let mut block_num = i +1;
                        let block_num_b = vec![(block_num >> 8) as u8, (block_num & 0xFF) as u8];
                        let packet_size = min(512, buffer.len() - i*512);
                        let packet_size_b = vec![(packet_size >> 8) as u8, (packet_size & 0xFF) as u8];
                        let mut bytes = vec![0x0u8, 0x3u8];
                        bytes.extend(packet_size_b);
                        bytes.extend(block_num_b);
                        bytes.extend(&buffer[i*512..(i*512 + packet_size)]);
                        writing_queue.push(bytes);
                    }
                    println!("File exists");
                    {connection_handler.lock().unwrap().send(bytes)};
                    {*state.await_ack.write().unwrap() = 0};
                    let mut block_num = 0;
                    // reverse the queue
                    let mut writing_queue: std::collections::VecDeque<Vec<u8>> = writing_queue.into_iter().collect();
                    while !writing_queue.is_empty() {
                        match rx.recv() {
                            Ok(i) => {
                                if i == block_num {
                                    let bytes = writing_queue.pop_front().unwrap();
                                    block_num += 1;
                                    {*state.await_ack.write().unwrap() = block_num as u16 };
                                    {connection_handler.lock().unwrap().send(bytes)};
                                    println!("Sent block {}", block_num);
                                    continue;
                                } else {
                                    println!("Wrong block number received");
                                    println!("Stopping WRQ file transfer");
                                    {*state.await_ack.write().unwrap() = 0};
                                    writing_queue.clear();
                                    return;
                                }
                            }
                            Err(_) => {
                                panic!("rx is poisoned")
                            }
                        }
                    }
                    match rx.recv() {
                        Ok(i) => {
                            println!("File sent");
                        }
                        Err(_) => {
                            panic!("rx is poisoned")
                        }
                    }
                }
                Err(e) => {
                    println!("Error opening file: {}", e);
                }
            }

        }
        "DELRQ" => {
            let mut bytes= vec![0x0u8, 0x8u8];
            let filename = args.clone();
            bytes.extend(filename.clone().into_bytes());
            {*state.await_ack.write().unwrap() = 0};
            {connection_handler.lock().unwrap().send(bytes)};
            match rx.recv() {
                Ok(0) => {
                    println!("ACK Received File deleted");
                }
                Ok(_) => {
                }
                Err(_) => {
                    panic!("rx is poisoned")
                }
            }

        }
        "DIRQ" => {
            let bytes= vec![0x0u8, 0x6u8];
            {*state.await_ack.write().unwrap() = 1};
            {*state.dirq.write().unwrap() = true};
            {*state.filename.write().unwrap() = "".to_string()};
            {connection_handler.lock().unwrap().send(bytes)};
            match rx.recv() {
                Ok(_) => {
                    {*state.dirq.write().unwrap() = false};
                }
                Err(_) => {
                    panic!("rx is poisoned")
                }
            }

        }
        "DISC" => {
            let bytes= vec![0x0u8, 0xAu8];
            {
                *state.await_ack.write().unwrap() = 0;
                *state.disc.write().unwrap() = true;
            }
            {connection_handler.lock().unwrap().send(bytes)};
            match rx.recv() {
                Ok(0) => {
                    {*state.connected.write().unwrap() = true;}
                    {*state.disc.write().unwrap() = false;}
                }
                Ok(_) => {
                    panic!("Invalid response for DISC from the server")
                }
                Err(_) => {
                    panic!("rx is poisoned")
                }
            }
        }
        &_ => {todo!("Invalid command")}
    }
}


pub fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let (ip, port) = if args.len() == 3 {(args[1].to_string(), args[2].to_string())} else {("127.0.0.1".to_string(), "7777".to_string())};
    let port = port.parse::<u16>().expect("port to be a unsigned 16 bit number");
    let mut connection_handler = Mutex::new(connection_handler::ConnectionHandler::new(ip, port));
    let (tx, rx) = std::sync::mpsc::channel::<i32>();
    let (tx2, rx2) = std::sync::mpsc::channel::<i32>();
    let state = Arc::new(State {
        terminate: RwLock::new(false),
        disc: RwLock::new(false),
        dirq: RwLock::new(false),
        await_ack: RwLock::new(0),
        connected: RwLock::new(false),
        filename: RwLock::new("".to_string()),
    });
    let state2 = state.clone();
    let arc = Arc::new(connection_handler);
    let arc2 = arc.clone();
    let t_lisent = std::thread::spawn(move || {
        loop {
            if *state.terminate.read().unwrap() {
                break;
            }
            let res = {arc.lock().unwrap().receive()};
            if let Some((bytes, opcode)) = res {
                println!("Received: {:x?}", bytes);
                listen(opcode, bytes, state.clone(), &tx, &rx2, &arc);
            }

        };
    });
    loop {
        if *state2.terminate.read().unwrap() {
            break;
        }
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let split: Vec<_> = input.split_whitespace().collect();
        let args = split[1..].join(" ");
        let command = split[0];
        if check_if_command_is_valid(command, split){
            handle_command(command, &args, &state2, &tx2, &rx, &arc2);
        }
    }
    t_lisent.join().unwrap();
    Ok(())
}