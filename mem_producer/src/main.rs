extern crate libc;
use std::any::Any;
use std::mem::ManuallyDrop;
use std::panic;
use std::ptr;
use std::result;
use std::slice;
use std::sync::Arc;
use std::sync::Mutex;

use libc::sleep;

struct Shm {
    id: i32,
    addr: *mut libc::c_void,
}

impl Drop for Shm {
    fn drop(&mut self) {
        println!("Shared memory release");
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
    let key: i32 = 2345;
    let shm_obj = Arc::new(Mutex::new(None));

    let shmid: i32;
    let shm: *mut libc::c_void;
    unsafe {
        shmid = libc::shmget(key, 46, libc::IPC_CREAT | 0644);
        if shmid == -1 {
            panic!("shmget failed: {}", std::io::Error::last_os_error());
        }
        shm = libc::shmat(shmid, ptr::null(), 0);
        if shm as isize == -1 {
            println!("shmat failed: {}", std::io::Error::last_os_error());
            libc::shmdt(shm);
            libc::shmctl(shmid, libc::IPC_RMID, ptr::null_mut());
            return;
        }
    }
    let shm_obj_inner = Shm {
        id: shmid,
        addr: shm,
    };

    let mut guard = shm_obj.lock().unwrap();
    let _ = std::mem::replace(&mut *guard, Some(shm_obj_inner)); // 锁定 Mutex，并设置共享数据

    let hello_world = b"this is the text we want to share through IPC\n";
    let slice = {
        let shm_addr = guard.as_mut().unwrap().addr as *mut u8; // 使用 guard 获取共享数据
        unsafe { std::slice::from_raw_parts_mut(shm_addr, hello_world.len()) }
    };
    slice.copy_from_slice(&hello_world[..]);

    println!("open for 10 secs");
    unsafe { sleep(10) };
}