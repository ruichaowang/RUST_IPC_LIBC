
use byteorder::{WriteBytesExt, LittleEndian};
use libc::{
    c_int, c_uint, c_void, msghdr, sendmsg, CMSG_DATA, CMSG_LEN, SCM_RIGHTS, SOL_SOCKET, iovec,
};
use std::fs::File;
use std::os::fd::IntoRawFd;
use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::UnixStream;
use std::thread::sleep;
use std::time::Duration;

/// 发送文件描述符
fn send_fd(fd: RawFd, stream: &mut UnixStream) -> std::io::Result<()> {
    let mut buffer = [0u8; 1];
    let mut iov = iovec { iov_base: buffer.as_mut_ptr() as *mut c_void, iov_len: 1 };

    let mut buf: [u8; 16] = [0; 16];
    let cmsg_space = unsafe { libc::CMSG_SPACE(std::mem::size_of::<c_int>() as u32) } as usize;
    
    let mut msg = msghdr {
        msg_name: std::ptr::null_mut(),
        msg_namelen: 0,
        msg_iov: &mut iov,
        msg_iovlen: 1,
        msg_control: buf.as_mut_ptr() as *mut c_void,
        msg_controllen: cmsg_space,
        msg_flags: 0,
    };

    let cmsg = unsafe { libc::CMSG_FIRSTHDR(&mut msg) };
    unsafe {
        (*cmsg).cmsg_level = SOL_SOCKET;
        (*cmsg).cmsg_type = SCM_RIGHTS;
        (*cmsg).cmsg_len = libc::CMSG_LEN(std::mem::size_of::<c_int>() as u32) as usize;
        *(libc::CMSG_DATA(cmsg) as *mut c_int) = fd;
    }

    let ret = unsafe { sendmsg(stream.as_raw_fd(), &mut msg, 0) };
    if ret < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
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
    let path = "/tmp/socket.sock";     // let path = "/system/bin/socket.sock";
    let file = File::open("/mnt/data/RUST_IPC_LIBC/socket/src/bin/testfile.txt")
        .expect("Failed to open file");
    let fd = file.into_raw_fd();
    // is_fd_valid(fd);
    let mut stream = UnixStream::connect(path).expect("failed to connect to socket");
    send_fd(fd, &mut stream).expect("failed to send fd");
    // stream.write_i32::<LittleEndian>(fd).expect("Failed to write integer to stream");
    // sleep(Duration::from_secs(10));
}