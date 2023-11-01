# RUST_IPC_LIBC

- Bionic目前并未实现System V IPC相关的功能，包括shmget()。它目前在mac 和ubuntu 上都需要sudo 才可以，
- 然而，POSIX共享内存的API（例如shm_open()）目前没有在Bionic libc中实现
- [SharedMemory Documentation](https://docs.rs/ndk/latest/ndk/shared_memory/struct.SharedMemory.html)
- 直接用fd 和size 去创建会失败，原因是：文件描述符是进程级别的资源，每个进程拥有自己的文件描述符表。因此，两个进程中的同一个文件描述符，可能指向两个完全不同的资源。不同进程间的文件描述符值，除非通过某种方式（如Unix socket的SCM_RIGHTS消息）显式传递，否则它们之间是没有关联的。
- 如果用 SharedMemory::create 创建，即使是同样的名字也读不到，
- fd 跨进程需要使用sendmsg 和recvmsg
- 共享内存传递值是可以的，但是指针不行， 你需要保证你的结构体是 Plain Old Data (POD) 类型的，即它不包含任何 Rust 语言特有的元素，例如引用、非空智能指针、枚举、结构体（除非这个结构体也是 POD 类型）、字符串等。

```[rust]
//不能直接把fd 当作i32 发送，原理是因为每个进程都有自己的fd
stream.write_i32::<LittleEndian>(fd)
    .expect("Failed to write integer to stream");
//同理，不能当成数字读取
stream.write_i32::<LittleEndian>(fd).expect("Failed to write integer to stream");
```

## 通过指针不能传递数据的原因

Storing a pointer in shared memory that points to some memory space within a process is meaningless for other processes. Even though shared memory is a mechanism that allows multiple processes to communicate and share information by accessing the same memory space, these processes still have their own separate, private address space.

A pointer stored in a shared memory segment that points to a location in one process's address space would not be meaningful (or indeed, valid) in any other process's address space. Pointers are typically used to reference locations in memory, but these locations are generally specific to individual processes. Every process has its own, distinct set of memory locations. So, if you create a pointer in one process and then try to dereference it in another process, you're likely to encounter errors because the location that the pointer is referencing in the second process does not contain the data that you expect it to contain.

Instead of storing pointers in shared memory, it's generally better to store data directly in the shared memory. That way, any process that has access to shared memory can directly access and work with the data as needed. They can do this without having to worry about dealing with pointers or trying to access memory locations in other processes's address space.

From your code, it seems that you would like to store `Engine` objects in shared memory so they can be accessed by different processes. Unfortunately, this is fundamentally not possible.

In general, you cannot directly share an object such as your `Engine` across processes by placing it in shared memory. This is because such an object relies on pointers (Rc and RefCell), and pointers are only valid within the context of a single process.

Instead, shared memory is typically used for sharing raw data such as integers, booleans, structs containing only POD (Plain Old Data), and arrays of such data. It cannot handle complex data types that include pointers or references.

Sharing complex objects such as `Engine` involves serializing the object into a format that can be safely written to shared memory. Then it must be deserialized in the other process. This could be very challenging with sophisticated objects and could limit the functionality of the object in the other processes.

For inter-process communication these are some alternatives:

1. JSON, XML or similar: the object is serialized to a string representation, and then parsed in the receiving process.

2. Protocol Buffers (protobuf): a language-neutral, platform-neutral extensible mechanism for serializing structured data.

3. Use an abstraction that hides the actual data sharing mechanism entirely.

But be aware, in all above cases you'll need to handle the engine initialization in each process separately, then synchronize state via shared memory or IPC.

Hope this gives you an idea how to proceed.

Unfortunately, you cannot apply these techniques directly to your Engine objects, especially if it involves GPU usage or other hardware aspects. You might need to architect your application so that Engine objects are created in each process, and share only necessary data between them.

In theory, it's possible to construct an object in shared memory that has been created using the mmap function. However, this only really works for simple, 'Plain Old Data' (POD) types that don't have a destructor, don't manage resources such as file descriptors or memory, and don't contain pointers or references to memory outside the mmaped region.

If you're determined to construct the object in shared memory and understand the risks, you could technically use placement new in C++ to construct an object in a specific memory location (though Rust doesn't directly support this). Be aware this is a very advanced and generally unsafe operation!

## 在指定mem 创建对象

```[cpp]
#include <iostream>
#include <new>  // required for the 'placement new' operator

struct MyStruct {
    int data;
    MyStruct(int data) : data(data) {}
};

int main() {
    // Reserving enough memory to store an instance of MyStruct
    char memory[sizeof(MyStruct)];

    // Using 'placement new' to construct an instance of MyStruct in that memory
    MyStruct* object = new (memory) MyStruct(42);

    std::cout << object->data << std::endl;  // prints: 42

    return 0;
}
```


```[rust]
use std::mem;
use std::alloc::{alloc, dealloc, Layout};

#[derive(Debug)]
struct MyStruct {
    data: i32,
}

fn main() {
    // Allocate memory for MyStruct
    let layout = Layout::new::<MyStruct>();
    let ptr = unsafe { alloc(layout) } as *mut MyStruct;

    // Use std::ptr::write for placement
    let my_struct = MyStruct { data: 42 };
    unsafe { std::ptr::write(ptr, my_struct) };

    // now, ptr points to a fully-constructed MyStruct
    unsafe {
        println!("{:?}", *ptr);  // prints: MyStruct { data: 42 }
    }

    // Cleanup
    unsafe { 
        std::ptr::drop_in_place(ptr);  // Manually drop the value
        dealloc(ptr as *mut u8, layout);  // Deallocate memory
    }
}
```
