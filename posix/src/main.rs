use libc::{
    c_char, c_uint, ftruncate, mmap, munmap, shm_open, shm_unlink, MAP_SHARED, O_CREAT, O_RDWR,
    PROT_READ, PROT_WRITE, S_IRGRP, S_IROTH, S_IRUSR, S_IWGRP, S_IWOTH, S_IWUSR,
};
use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::mem::size_of;
use std::os::unix::prelude::FromRawFd;
use std::ptr::null_mut;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mem_size = size_of::<i32>();
    let name = CString::new("/test2.shm").unwrap();
    let c_str_name: *const c_char = name.as_ptr();
    let fd = unsafe {
        shm_open(
            c_str_name,
            O_CREAT | O_RDWR,
            (S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH) as c_uint,
        )
    };

    unsafe {
        let _ = ftruncate(fd, mem_size as _);
    }

    let ptr = unsafe {
        mmap(
            null_mut(),
            mem_size,
            PROT_READ | PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        )
    };

    let file = unsafe { File::from_raw_fd(fd) };
    file.set_len(mem_size as _).unwrap();

    unsafe {
        *(ptr as *mut i32) = 100;
        let mut file = file;
        let mut buffer = [0; 4];
        file.read(&mut buffer).unwrap();
        let val = i32::from_ne_bytes(buffer);
        assert_eq!(100, val);

        munmap(ptr, mem_size);
    }

    sleep(Duration::from_secs(20));
    unsafe { shm_unlink(c_str_name) };
    println!("Close!")
}
