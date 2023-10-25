use byteorder::{LittleEndian, ReadBytesExt};
use std::os::unix::net::UnixListener;

fn main() {
    let path = "/system/bin/socket.sock";
    let listener = UnixListener::bind(path).expect("failed to bind socket");
    for stream in listener.incoming() {
        let mut stream = stream.expect("failed to accept connection");
        let number = stream
            .read_i32::<LittleEndian>()
            .expect("Failed to read integer from stream");
        println!("Received number: {}", number);
        std::fs::remove_file(path).unwrap();  // 删除旧的套接字，如果存在的话

        break;
    }
}