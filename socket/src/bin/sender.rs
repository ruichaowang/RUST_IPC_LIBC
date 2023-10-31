
use byteorder::{WriteBytesExt, LittleEndian};
use libc::{
    c_int, c_uint, c_void, msghdr, sendmsg, CMSG_DATA, CMSG_LEN, SCM_RIGHTS, SOL_SOCKET,
};
use std::fs::File;
use std::os::fd::IntoRawFd;
use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::UnixStream;
use std::thread::sleep;
use std::time::Duration;

/// 发送文件描述符，但是这个方法有问题，接收方接收到的文件描述符空的
fn send_fd(fd: RawFd, stream: &mut UnixStream) -> std::io::Result<()> {
    // 准备发送的消息
    let cmsg_space = unsafe { libc::CMSG_SPACE(std::mem::size_of::<c_int>() as c_uint) as usize };
    println!("Control message space: {}", cmsg_space);

    let mut buf: [u8; 16] = [0; 16]; // Assuming 128 bytes is large enough to accommodate control message.
    let mut hdr = msghdr {
        msg_name: std::ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: std::ptr::null_mut(),
        msg_iovlen: 0,
        msg_control: buf.as_mut_ptr() as *mut c_void,
        msg_controllen: cmsg_space, //as u32
        msg_flags: 0,
    };

    let cmsg = unsafe { libc::CMSG_FIRSTHDR(&mut hdr) };
    if cmsg.is_null() {
        println!("No control message found");
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "No control message found"));
    } else {
        println!("Control message found");
    }

    println!("2");

    unsafe {
        if !cmsg.is_null() {
            (*cmsg).cmsg_level = SOL_SOCKET;
            (*cmsg).cmsg_type = SCM_RIGHTS;
            (*cmsg).cmsg_len = CMSG_LEN((std::mem::size_of::<c_int>() as usize).try_into().unwrap()) as usize; //as u32
            let fd_ptr = CMSG_DATA(cmsg) as *mut c_int;
            std::ptr::write(fd_ptr, fd);
        }
    }

    println!("3");

    // 发送消息
    let ret = unsafe { sendmsg(stream.as_raw_fd(), &hdr, 0) };
    if ret < 0 {
        return Err(std::io::Error::last_os_error());
    }

    println!("4");

    Ok(())
}

fn is_fd_valid(fd: RawFd) {
    let ret = unsafe { libc::fcntl(fd, libc::F_GETFL) };
    if ret == -1 {
        panic!("Invalid fd");
    } else {
        println!("Valid fd");
    }
}

fn main() {
    let path = "/system/bin/socket.sock";
    let file = File::open("/system/bin/test.txt")
        .expect("Failed to open file");
    let fd = file.into_raw_fd();
    is_fd_valid(fd);
    let mut stream = UnixStream::connect(path).expect("failed to connect to socket");
    send_fd(fd, &mut stream).expect("failed to send fd");
    sleep(Duration::from_secs(20));
}