use ndk::shared_memory::SharedMemory;
use std::thread::sleep;
use std::{ffi::CString, mem::size_of, os::fd::AsRawFd, time::Duration};

fn main() {
    // 我需要在这部分写入一个数字，然后在另一个进程中读取
    let mem_size = size_of::<i32>();
    let name = CString::new("/test2.shm").unwrap();
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


    let buffer_slice = unsafe { std::slice::from_raw_parts_mut(buffer.cast(), mem_size) };
    let number: i32 = 100;
    buffer_slice.copy_from_slice(&number.to_le_bytes());

    // limit access to read only
    let _ = mem.set_prot(libc::PROT_READ);
    println!("write done!");
    sleep(Duration::from_secs(10));  //only open for 10s

    // Existing mappings will retain their protection flags (PROT_WRITE here) after set_prod()
    // unless it is unmapped:
    // unsafe { libc::munmap(buffer, size) };
    println!("shared mem close!");
    
}
