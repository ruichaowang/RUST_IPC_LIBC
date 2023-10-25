use std::os::unix::net::UnixStream;
use byteorder::{WriteBytesExt, LittleEndian};
use std::io::Write;
use std::fs;

fn main() {
    fs::remove_file("/tmp/socket.sock").unwrap();  // 删除旧的套接字，如果存在的话

    let mut stream = UnixStream::connect("/tmp/socket.sock").expect("failed to connect to socket");
    stream.write_i32::<LittleEndian>(127).expect("Failed to write integer to stream");
}