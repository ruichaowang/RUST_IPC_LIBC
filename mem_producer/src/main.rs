extern crate libc;
use std::ffi::CString;
use std::ptr;
use std::slice;

use libc::sleep;

struct Shm {
    id: i32,
    addr: *mut libc::c_void,
}

impl Drop for Shm {
    fn drop(&mut self) {
        unsafe {
            // detaching the shared memory segment
            if libc::shmdt(self.addr) == -1 {
                eprintln!("shmdt failed: {}", std::io::Error::last_os_error());
            }

            // deleting the shared memory segment
            if libc::shmctl(self.id, libc::IPC_RMID, ptr::null_mut()) == -1 {
                eprintln!("shmctl failed: {}", std::io::Error::last_os_error());
            }
        }
    }
}

fn main() {
    println!("Opening shared memory...");
    let key = 5678;
    let shmid: i32;
    let shm: *mut libc::c_void;
    unsafe {
        shmid = libc::shmget(key, 46 , libc::IPC_CREAT | 0666);
        if shmid == -1 {
            panic!("shmget failed: {}", std::io::Error::last_os_error());
        }
        shm = libc::shmat(shmid, ptr::null(), 0);
        if shm as isize == -1 {
            panic!("shmat failed: {}", std::io::Error::last_os_error());
        }
    }
    let shm_obj = Shm {
        id: shmid,
        addr: shm,
    };


    let hello_world = b"this is the text we want to share through IPC\n";
    let slice = unsafe { slice::from_raw_parts_mut(shm_obj.addr as *mut u8, hello_world.len()) };
    slice.copy_from_slice(&hello_world[..]);

    unsafe { sleep(20) };
    println!("closing shared memory...");
}
