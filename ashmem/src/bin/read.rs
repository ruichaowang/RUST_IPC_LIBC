use ndk::shared_memory::SharedMemory;
use std::{
    ffi::CString,
    fs::File,
    io::Read,
    mem::size_of,
    os::fd::{AsRawFd, FromRawFd},
};

fn main() {
    let mem_size = std::cmp::max(size_of::<i32>(), b"hello!\0".len());
    let name = CString::new("/test2.shm").unwrap();

    let shared_mem = SharedMemory::create(Some(&name), mem_size).unwrap();
    let size = shared_mem.size();

    let fd = shared_mem.as_raw_fd();
    if fd == -1 {
        panic!("fd failed");
    }
    let ptr = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            mem_size,
            libc::PROT_READ,
            libc::MAP_SHARED,
            fd,
            0,
        )
    };
    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }
    let mut file = unsafe { File::from_raw_fd(shared_mem.as_raw_fd()) };

    let mut buffer = [0; 4];
    file.read_exact(&mut buffer).expect("read failed");
    let val = i32::from_ne_bytes(buffer);
    println!("val: {}", val);
    // Clean up the memory mapping
    unsafe { libc::munmap(ptr, size) };
}
