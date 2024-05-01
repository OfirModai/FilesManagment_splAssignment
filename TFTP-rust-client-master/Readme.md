## How to use
1. Download form release here: [Download](https://github.com/bguspl/TFTP-rust-client/releases) for your os.
2. Run the program with the following command in the binary directory:
```bash
./Tftpclient <server-ip> <port> # the name is different in each release so look for the binary name
```
3. flow the instruction on the assiment page for client behavior.
4. GLHF

## How to build
1. Clone the repository or download the source code From code button on the top right -> download zip.
2. install rust from [here](https://www.rust-lang.org/tools/install) or just run the following command (for macOS and linux):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3. cargo build --release
4. The binary will be in the target/release/Tftpclinet
5. GLHF

## VSCode devcontainer to compile
The repository contains a devcontainer for vscode, just open the repository in vscode and click in the bottom right corner on the green icon and select "Reopen in container" and you are good to go.

The docker container is the same as the assignment container but also include rust, so you can compile the code and just copy the binary to your host machine.

## Notes:
The program have more print than the required for the assignment, this is for debugging and understanding the flow of the program.

The received bytes are printed in HEX format for you to see them.

We don't expect you to read this code and understand it, the point is not to give you a code example just a binary to test your server.

### Any question about the code in the forum will be ignored.

The code is not 1 to 1 with the assignment and may deviate from the assignment in some places.

**So if there is any deviation from the assignment instruction your implementation should follow the instruction**

**Any appeals about this subject will be dismissed**

## Known issues
- The program is not tested on windows, so it may not work on windows.
- The code is tested on macOS and devcontainer, so it should work on ubuntu 20.04
