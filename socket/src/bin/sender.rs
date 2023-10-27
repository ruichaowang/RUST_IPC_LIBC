use libc::{
    c_int, c_uint, c_void, iovec, msghdr, sendmsg, CMSG_DATA, CMSG_LEN, SCM_RIGHTS, SOL_SOCKET,
};
use std::fs::File;
use std::os::fd::IntoRawFd;
use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::UnixStream;

fn send_fd(fd: RawFd, stream: &mut UnixStream) -> std::io::Result<()> {
    // 准备发送的消息
    println!("1");
    let mut iov = iovec {
        iov_base: std::ptr::null_mut(),
        iov_len: 0,
    };

    let cmsg_space = unsafe { libc::CMSG_SPACE(std::mem::size_of::<c_int>() as c_uint) as usize };

    println!("2");
    let mut buf: [u8; 128] = [0; 128];  // Assuming 128 bytes is large enough to accommodate control message.
    let mut hdr = msghdr {
        msg_name: std::ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: buf.as_mut_ptr() as *mut c_void,
        msg_controllen: cmsg_space as u32,
        msg_flags: 0,
    };


    println!("3");
    let cmsg = unsafe { libc::CMSG_FIRSTHDR(&mut hdr) };
    unsafe {
        if !cmsg.is_null() {
            (*cmsg).cmsg_level = SOL_SOCKET;
            (*cmsg).cmsg_type = SCM_RIGHTS;
            (*cmsg).cmsg_len = CMSG_LEN(std::mem::size_of::<c_int>() as u32);
            let fd_ptr = CMSG_DATA(cmsg) as *mut c_int;
            std::ptr::write(fd_ptr, fd);
        }
    }

    println!("4");

    // 发送消息
    let ret = unsafe { sendmsg(stream.as_raw_fd(), &hdr, 0) };
    println!("5");
    if ret < 0 {
        return Err(std::io::Error::last_os_error());
    }

    Ok(())
}

fn main() {
    let path = "/tmp/socket.sock";
    let file = File::open("/Users/wangruichao/Downloads/W01_fake_no_ext.zip")
        .expect("Failed to open file");
    let fd = file.into_raw_fd();
    let mut stream = UnixStream::connect(path).expect("failed to connect to socket");
    send_fd(fd, &mut stream).expect("failed to send fd");
}

// stream.write_i32::<LittleEndian>(12).expect("Failed to write integer to stream");
