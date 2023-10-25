use std::os::unix::net::UnixStream;
use byteorder::{WriteBytesExt, LittleEndian};

fn main() {
    let path = "/system/bin/socket.sock";
    let mut stream = UnixStream::connect(path).expect("failed to connect to socket");
    stream.write_i32::<LittleEndian>(12).expect("Failed to write integer to stream");
}