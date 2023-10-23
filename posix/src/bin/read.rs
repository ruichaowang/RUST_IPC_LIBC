// 这是从共享内存中读取数据的代码
use std::ffi::CString;
use std::os::unix::prelude::FromRawFd;
use std::fs::File;
use nix::sys::mman::{shm_open, munmap, mmap, MapFlags, ProtFlags};
use nix::fcntl::OFlag;
use nix::sys::stat::Mode;
use std::ptr::null_mut;
use std::mem::size_of;
use std::io::Read;

fn main() {
    let shared_name = CString::new("/test2.shm").expect("CString::new failed");
    let name_c_str = shared_name.as_c_str();
    let mem_size: usize = size_of::<i32>();

    // Open the existing shared memory and get the file descriptor
    let fd = shm_open(name_c_str, OFlag::O_RDWR, Mode::empty()).expect("shm_open failed");

    // Map the shared memory into the address space of this process
    let ptr = unsafe { mmap(null_mut(), mem_size, ProtFlags::PROT_READ, MapFlags::MAP_SHARED, fd, 0).expect("mmap failed") };

    // Operate on the shared memory
    let mut file = unsafe { File::from_raw_fd(fd) };

    let mut buffer = [0; 4];
    file.read(&mut buffer).expect("read failed");
    let val = i32::from_ne_bytes(buffer);
    assert_eq!(val, 100);
    println!("val: {}", val);

    // This process finished using this shared memory, so do unmap
    unsafe { munmap(ptr, mem_size).expect("munmap failed") };
}