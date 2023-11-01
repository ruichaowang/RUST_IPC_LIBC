adb root && adb remount
cd ~/Work/Rust/shared_mem/ashmem
cargo build --target aarch64-linux-android
adb push target/aarch64-linux-android/debug/create /system/bin
adb push target/aarch64-linux-android/debug/read /system/bin
adb shell /system/bin/read &
adb shell /system/bin/create 