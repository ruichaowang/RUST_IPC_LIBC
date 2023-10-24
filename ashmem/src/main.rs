use std::{ffi::CString, mem::size_of, os::fd::AsRawFd};
use ndk::shared_memory::SharedMemory;

fn main() {
    println!("Hello, world!");
    let mem_size = std::cmp::max(size_of::<i32>(), b"hello!\0".len());
    let name = CString::new("/test2.shm").unwrap();

    let mem = SharedMemory::create(Some(&name), mem_size).unwrap();
    let size = mem.size();
    let buffer = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            size,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            mem.as_raw_fd(),
            0,
        )
    };

    let buffer_slice = unsafe { std::slice::from_raw_parts_mut(buffer.cast(), size) };
    buffer_slice[..7].copy_from_slice(b"hello!\0");

    // Existing mappings will retain their protection flags (PROT_WRITE here) after set_prod()
    // unless it is unmapped:
    unsafe { libc::munmap(buffer, size) };

    // limit access to read only
    let _ = mem.set_prot(libc::PROT_READ);
}
