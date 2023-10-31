# RUST_IPC_LIBC

- Bionic目前并未实现System V IPC相关的功能，包括shmget()。它目前在mac 和ubuntu 上都需要sudo 才可以，
- 然而，POSIX共享内存的API（例如shm_open()）目前没有在Bionic libc中实现
- [SharedMemory Documentation](https://docs.rs/ndk/latest/ndk/shared_memory/struct.SharedMemory.html)

```[rust]
//不能直接把fd 当作i32 发送，原理是因为每个进程都有自己的fd
stream.write_i32::<LittleEndian>(fd)
    .expect("Failed to write integer to stream");
//同理，不能当成数字读取
let number = sock
    .read_i32::<LittleEndian>()
    .expect("Failed to read integer from stream");
println!("Received number: {}", number);
```
