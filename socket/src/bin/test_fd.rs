use std::{fs::File, os::fd::IntoRawFd};

fn main() {
    let file = File::open("/Users/wangruichao/Downloads/test.txt")
        .expect("Failed to open file");
    let fd = file.into_raw_fd();
}