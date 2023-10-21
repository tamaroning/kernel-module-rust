# kernel-module-rust

Try Rust for Linux

```
% docker compose up -d
% docker exec -it kernel-module-rust-ubuntu-1 bash
# cd /kernel-module-rust/rust_kernel
# make
# sudo insmod hello_rust.ko
# sudo dmesg | tail -1
<Message from kernel module>
# sudo rmmod hello_rust
```

## References
- https://github.com/Rust-for-Linux/linux/tree/rust-next/Documentation/rust
- https://dev.classmethod.jp/articles/linuxkernel-with-rust/
