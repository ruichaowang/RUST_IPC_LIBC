use libc::{c_int, c_void, iovec, msghdr, recvmsg};
use std::os::unix::io::{AsRawFd, RawFd};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::thread;
use std::time::Duration;

fn recv_fd(sock: &mut UnixStream) -> std::io::Result<RawFd> {
    let mut buffer = [0u8; 1];
    let mut iov = iovec {
        iov_base: buffer.as_mut_ptr() as *mut c_void,
        iov_len: 1,
    };

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

    let ret = unsafe { recvmsg(sock.as_raw_fd(), &mut msg, 0) };
    if ret <= 0 {
        Err(std::io::Error::last_os_error())
    } else {
        let cmsg = unsafe { libc::CMSG_FIRSTHDR(&mut msg) };
        if cmsg.is_null() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No file descriptor received",
            ))
        } else {
            Ok(unsafe { *(libc::CMSG_DATA(cmsg) as *mut c_int) })
        }
    }
}

fn print_waiting() {
    for _ in 1..6 {
        // 5秒的总等待时间, 修改这个范围以改变等待时间
        println!("Waiting for incoming connection...");
        thread::sleep(Duration::from_secs(1)); //线程休眠1秒
    }
}

fn main() {
    let path = "/system/bin/socket.sock";
    // let path = "/tmp/socket.sock";
    if Path::new(&path).exists() {
        std::fs::remove_file(path).expect("Failed to remove file");
    }
    let listener = UnixListener::bind(path).expect("failed to bind socket");

    loop {
        print_waiting(); // 打印提示
        match listener.accept() {
            Ok((mut sock, _)) => {
                let fd = recv_fd(&mut sock);
                //   let fd = sock.read_i32::<LittleEndian>();
                match fd {
                    Ok(fd) => {
                        println!("Successfully received fd: {}", fd);
                        //添加你的文件操作代码...

                        // 一旦我们成功地从一个连接中接收到文件，我们可以跳出循环
                        break;
                    }
                    Err(e) => {
                        println!("Failed to receive fd: {}", e);
                        break;
                    }
                }
            }
            Err(err) => {
                println!("Failed to accept socket connection: {}", err);
            }
        }
    }
}
