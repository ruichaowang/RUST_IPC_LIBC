use libc::{c_int, c_void, iovec, msghdr, sendmsg, SCM_RIGHTS, SOL_SOCKET};
use ndk::shared_memory::SharedMemory;
use std::os::unix::io::RawFd;
use std::os::unix::net::UnixStream;
use std::thread::sleep;
use std::{ffi::CString, mem::size_of, os::fd::AsRawFd, time::Duration};

use std::mem::{MaybeUninit};

#[repr(C)]
pub struct MyStruct {
    pub data1: i32,
    pub data2: f32,
}

fn create_shared_memory_struct() -> std::io::Result<()> {
    let mem_size = size_of::<MyStruct>();
    let name = CString::new("/test_shared_mem.shm").unwrap();
    let mem = SharedMemory::create(Some(&name), mem_size).unwrap();
    let buffer = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            mem_size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            mem.as_raw_fd(),
            0,
        )
    };

    if buffer == libc::MAP_FAILED {
        return Err(std::io::Error::last_os_error());
    }

    let my_struct = MyStruct {
        data1: 100,
        data2: 2.2,
    };

    // 安全地将 my_struct 写入 buffer
    unsafe { *(buffer as *mut MyStruct) = MaybeUninit::new(my_struct).assume_init() }

    // limit access to read only
    let _ = mem.set_prot(libc::PROT_READ);

    // send fd to another process
    let path = "/system/bin/socket.sock";
    let mut stream = UnixStream::connect(path).expect("failed to connect to socket");
    send_fd(mem.as_raw_fd(), &mut stream).expect("failed to send fd");

    sleep(Duration::from_secs(10));
    println!("shared mem close!");

    Ok(())
}
/// 发送文件描述符
fn send_fd(fd: RawFd, stream: &mut UnixStream) -> std::io::Result<()> {
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

/// this process is for write a value 100 to shared memory
fn main() {
    let _ = create_shared_memory_struct();
}
