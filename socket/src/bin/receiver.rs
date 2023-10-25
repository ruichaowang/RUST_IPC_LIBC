use byteorder::{LittleEndian, ReadBytesExt};
use std::fs;
use std::fs::File;
use std::os::unix::net::UnixListener;

fn main() {
    // 删除旧的套接字，如果存在的话
    fs::remove_file("/tmp/socket.sock").unwrap();
    //在初始化监听之前，请确保套接字文件已经创建并存在于正确的路径上。
    File::create("/tmp/socket.sock").expect("Could not create socket file");

    let listener = UnixListener::bind("/tmp/socket.sock").expect("failed to bind socket");
    for stream in listener.incoming() {
        let mut stream = stream.expect("failed to accept connection");
        let number = stream
            .read_i32::<LittleEndian>()
            .expect("Failed to read integer from stream");
        println!("Received number: {}", number);
        break;
    }
}
