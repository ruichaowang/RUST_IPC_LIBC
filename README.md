# RUST_IPC_LIBC

- Bionic目前并未实现System V IPC相关的功能，包括shmget()。它目前在mac 和ubuntu 上都需要sudo 才可以，
- 然而，POSIX共享内存的API（例如shm_open()）目前没有在Bionic libc中实现
- [SharedMemory Documentation](https://docs.rs/ndk/latest/ndk/shared_memory/struct.SharedMemory.html)
- 直接用fd 和size 去创建会失败，原因是：文件描述符是进程级别的资源，每个进程拥有自己的文件描述符表。因此，两个进程中的同一个文件描述符，可能指向两个完全不同的资源。不同进程间的文件描述符值，除非通过某种方式（如Unix socket的SCM_RIGHTS消息）显式传递，否则它们之间是没有关联的。

- fd 跨进程需要使用sendmsg 和recvmsg

```[rust]
//不能直接把fd 当作i32 发送，原理是因为每个进程都有自己的fd
stream.write_i32::<LittleEndian>(fd)
    .expect("Failed to write integer to stream");
//同理，不能当成数字读取
stream.write_i32::<LittleEndian>(fd).expect("Failed to write integer to stream");
```
