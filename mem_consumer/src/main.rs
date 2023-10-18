extern crate libc;
use std::ffi::CString;
use std::ptr;
use std::slice;

fn main() {
    unsafe {
        let key = 5678;
        let shmid = libc::shmget(key, 0, 0666);
        let shm = libc::shmat(shmid, ptr::null(), 0);
        let slice = slice::from_raw_parts(shm as *const u8, 46);
        let message = std::str::from_utf8_unchecked(slice);
        println!("{}", message);
    }
}