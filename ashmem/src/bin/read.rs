use std::mem::size_of;
/// 如果用 SharedMemory::create 创建，即使是同样的名字也读不到，
/// 直接用fd 和size 去创建会失败，原因是：文件描述符是进程级别的资源，每个进程拥有自己的文件描述符表。因此，两个进程中的同一个文件描述符，可能指向两个完全不同的资源。不同进程间的文件描述符值，除非通过某种方式（如Unix socket的SCM_RIGHTS消息）显式传递，否则它们之间是没有关联的。
fn main() {
    let fd = 3;
    let mem_size = size_of::<i32>();
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

    // 这里我们直接从内存中读取i32值
    let val = unsafe { *(ptr as *mut i32) };
    println!("val = {}", val);

    // Clean up the memory mapping
    unsafe { libc::munmap(ptr, mem_size) };
}
