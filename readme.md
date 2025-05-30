### how to run

```shell
rustup override set nightly &&
cargo install bootimage &&
rustup component add llvm-tools-preview &&
cargo bootimage
```

```shell
sudo apt update && sudo apt install qemu-system
```

```shell
qemu-system-x86_64 -drive format=raw,file=target/x86_64-osel/debug/bootimage-OSel.bin
```