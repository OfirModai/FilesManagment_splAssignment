use std::io::Bytes;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode{
    None, RRQ, WRQ, DATA, ACK, ERROR, DIRQ, LOGRQ, DELRQ, BCAST, DISC
}

impl Opcode {
    pub fn from_u16(opcode: u16) -> Opcode {
        match opcode {
            1 => Opcode::RRQ,
            2 => Opcode::WRQ,
            3 => Opcode::DATA,
            4 => Opcode::ACK,
            5 => Opcode::ERROR,
            6 => Opcode::DIRQ,
            7 => Opcode::LOGRQ,
            8 => Opcode::DELRQ,
            9 => Opcode::BCAST,
            10 => Opcode::DISC,
            _ => Opcode::None
        }
    }
}

#[derive(Debug)]
pub(crate) struct EenDec {
    bytes: Vec<u8>,
    opcode: Opcode,
    opt_expected_len : u32,
}

impl EenDec {
    pub(crate) fn new() -> EenDec {
        EenDec {
            bytes: Vec::new(),
            opcode: Opcode::None,
            opt_expected_len: std::u32::MAX,
        }
    }

    pub fn pool_bytes(&mut self) -> Vec<u8> {
        let mes = self.bytes.clone();
        self.bytes.clear();
        self.set_opcode(Opcode::None);
        mes
    }

    fn set_opcode(&mut self, opcode: Opcode) {
        self.opcode = opcode;
        self.opt_expected_len = match opcode {
            Opcode::None => {std::u32::MAX}
            Opcode::RRQ | Opcode::WRQ | Opcode::DIRQ
            | Opcode::LOGRQ | Opcode::DELRQ | Opcode::DISC  => {2}
            Opcode::BCAST => {3}
            Opcode::ACK | Opcode::ERROR=> {4}
            Opcode::DATA => {6}
        };

    }

    fn get_opcode(&self) -> Opcode {
        self.opcode.clone()
    }

    fn peek_opcode(&self) -> Opcode {
        assert!(self.bytes.len() >= 2);
        let u16_opcode = (self.bytes[0] as u16) << 8 | self.bytes[1] as u16;
        Opcode::from_u16(u16_opcode)
    }


    pub fn encode(&self, bytes: Vec<u8>) -> Vec<u8> {
        let opcode = Opcode::from_u16((bytes[0] as u16) << 8 | bytes[1] as u16);
        match opcode {
            Opcode::None => {panic!("Invalid opcode")}
            Opcode::RRQ | Opcode::WRQ | Opcode::ERROR | Opcode::BCAST | Opcode::LOGRQ | Opcode::DELRQ
                => {let mut res = Vec::new();
                res.extend(bytes);
                res.push(0x0);
                res}
            Opcode::ACK | Opcode::DIRQ | Opcode::DISC | Opcode::DATA => {bytes}
        }
    }

    fn have_added_zero(opcode: &Opcode) -> bool {
        match opcode {
            Opcode::RRQ | Opcode::WRQ | Opcode::ERROR
            | Opcode::BCAST | Opcode::LOGRQ | Opcode::DELRQ | Opcode::None => {true}
            _ => {false}
        }
    }

    pub fn decode(&mut self, nextbyte: u8) -> Option<(Vec<u8>, Opcode)> {
        if self.bytes.len() >= self.opt_expected_len as usize && nextbyte == 0x0 {
        let opcode = self.get_opcode();
        let mes = self.pool_bytes();
        self.set_opcode(Opcode::None);
        Some((mes, opcode))
        }
         else {
            self.bytes.push(nextbyte);
            if self.bytes.len() == 2 {
                self.set_opcode(self.peek_opcode());
            }
            if self.opcode == Opcode::DATA && self.bytes.len() == 4 {
                let size = (self.bytes[2] as u16) << 8 | self.bytes[3] as u16;
                self.opt_expected_len = 6 + size as u32;
            }
             if !Self::have_added_zero(&self.opcode) && self.bytes.len() == (self.opt_expected_len as usize)
             {
                 let opcode = self.get_opcode();
                 let mes = self.pool_bytes();
                 self.set_opcode(Opcode::None);
                 return Some((mes, opcode));
             }
            None
        }
    }
}