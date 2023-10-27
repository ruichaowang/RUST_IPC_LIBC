use byteorder::{ReadBytesExt, LittleEndian};
use libc::{c_int, c_void, iovec, msghdr, recvmsg};
use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;

fn recv_fd(sock: &UnixStream) -> std::io::Result<RawFd> {
    // 接收消息
    let mut buf = [0u8; 1024];
    let mut iov = iovec {
        iov_base: buf.as_mut_ptr() as *mut c_void,
        iov_len: buf.len(),
    };

    let mut hdr = msghdr {
        msg_name: std::ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: std::ptr::null_mut(),
        msg_controllen: 0,
        msg_flags: 0,
    };

    let ret = unsafe { recvmsg(sock.as_raw_fd(), &mut hdr, 0) };
    if ret < 0 {
        return Err(std::io::Error::last_os_error());
    }

    // 解析消息
    let cmsg = unsafe { libc::CMSG_FIRSTHDR(&hdr) };
    if cmsg.is_null() {
        println!("No file descriptor received");
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No file descriptor received"));
    } else {
        println!("File descriptor received");
    }

    let fd_ptr = unsafe { libc::CMSG_DATA(cmsg) as *const c_int };
    let fd = unsafe { std::ptr::read(fd_ptr) };
    println!("4");
    Ok(fd)
}

fn main() {
    // let path = "/system/bin/socket.sock";
    let path = "/tmp/socket.sock";
    if Path::new(&path).exists() {
        std::fs::remove_file(path).expect("Failed to remove file");
    }

    let listener = UnixListener::bind(path).expect("failed to bind socket");
    let (mut sock, _) = listener.accept().expect("accept error");
    
    let number = sock
        .read_i32::<LittleEndian>()
        .expect("Failed to read integer from stream");
    println!("Received number: {}", number);

    // let fd = recv_fd(&sock).expect("Failed to receive fd");
    // println!("Received fd: {}", fd);
}
