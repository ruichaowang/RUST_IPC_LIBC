use libc::{
    c_char, c_uint, mmap, munmap, shm_open, O_RDWR, PROT_READ, S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP,
    S_IWOTH, S_IWUSR,
};
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::os::unix::prelude::FromRawFd;
use std::ptr::null_mut;

fn main() {
    let shared_name = CString::new("/test2.shm").expect("CString::new failed");
    let mem_size = std::mem::size_of::<i32>();
    let c_str_name: *const c_char = shared_name.as_ptr();

    let fd = unsafe {
        shm_open(
            c_str_name,
            O_RDWR,
            (S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH) as c_uint,
        )
    };
    if fd == -1 {
        panic!("shm_open failed");
    }

    let ptr = unsafe { mmap(null_mut(), mem_size, PROT_READ, libc::MAP_SHARED, fd, 0) };
    if ptr == libc::MAP_FAILED {
        panic!("mmap failed");
    }

    let mut file = unsafe { File::from_raw_fd(fd) };

    let mut buffer = [0; 4];
    file.read_exact(&mut buffer).expect("read failed");
    let val = i32::from_ne_bytes(buffer);
    assert_eq!(val, 100);
    println!("val: {}", val);

    unsafe {
        munmap(ptr, mem_size);
    }
}
