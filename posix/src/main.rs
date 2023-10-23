// use nix::fcntl::OFlag;//ftruncate 当前nix 中没有
use nix::fcntl::{OFlag, ftruncate};
use nix::sys::mman::{mmap, munmap, shm_open, shm_unlink, MapFlags, ProtFlags};
use nix::sys::stat::Mode;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Read};
use std::mem::size_of;
use std::os::unix::prelude::FromRawFd;
use std::ptr::null_mut;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mem_size = size_of::<i32>();
    let name = CString::new("/test2.shm").unwrap();
    let c_str_name: &CStr = name.as_c_str();
    let fd = shm_open(c_str_name, OFlag::O_CREAT | OFlag::O_RDWR, Mode::empty()).unwrap();

    let _ = ftruncate(fd, mem_size as _);
    // unsafe {
    //     libc::ftruncate(fd, mem_size as _);
    // }

    let ptr = unsafe {
        mmap(
            null_mut(),
            mem_size,
            ProtFlags::PROT_READ | ProtFlags::PROT_WRITE,
            MapFlags::MAP_SHARED,
            fd,
            0,
        )
        .unwrap()
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

        munmap(ptr, mem_size).unwrap();
    }

    shm_unlink(c_str_name).unwrap();
    sleep(Duration::from_secs(10));
}
