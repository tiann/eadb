# eadb - eBPF Android Debug Bridge 

eadb provides a powerful Linux shell environment where one can run [BCC](https://github.com/iovisor/bcc) / [bpftrace](https://github.com/iovisor/bpftrace) / [bpftool](https://github.com/libbpf/bpftool) on Android device.

## Usage

Install the eadb first, please refer [Install](https://github.com/tiann/eadb#install)

### Prepare eadb environment on Android device

eadb support two mode to connect the device:

- adb
- ssh

Both of them need the root privilege.

If you want to use `adb` mode, `adb root` is required but `adb root` is disabled in production build, if you doesn't have any userdebug / eng devices, you can try [adb_root](https://github.com/tiann/adb_root). But it you don't any experience on Magisk(it may brick your device), it is recommended to use `ssh` mode.

If you use `ssh` mode, it is recommended to install [Magisk](https://github.com/topjohnwu/Magisk) to Root your device and install [MagiskSSH](https://gitlab.com/d4rcm4rc/MagiskSSH_releases) to enable ssh.

#### Download it from github

When you can use adb or ssh to connect to your device, you can prepare the eadb environment:

```sh
eadb --ssh root@ip prepare
```

The command would download a rootfs from [Release page](https://github.com/tiann/eadb/releases) and push it to your device, then do some mounts and chroot in to the environment.

#### Use an existing archive

You can also download or build the rootfs and then prepare it by your rootfs file:

```sh
eadb --ssh root@ip prepare -a path/to/archive
```

### Enter the environment

```sh
eadb --ssh root@ip shell
```

You will enter the eadb environment and get a shell by this command, you can use `apt update` to update the sources and install softwares(such as clang,llvm,bpftrace) by yourself, you can even install `Rust` / `Golang` or `gcc` to do development on this device!

### Build the environment by yourself

Only Ubuntu / Debian is supported to build the system image running on Android, you can use docker or podman on macOS and WSL on Windows.

Install `qemu-user-static` and `debootstrap` first:

```sh
sudo apt update && sudo apt install qemu-user-static debootstrap
```

And then build the eadb (root is required):

```sh
sudo eadb build
```

After the build, you will get a `debianfs-full(mini).tar.gz` in your working directory. you can use this image as your environment:

```sh
eadb --ssh root@ip prepare -a deb.tar.gz
```

## Install

### Binary

Download binaries in [Release page](https://github.com/tiann/eadb/releases)

### Build from source

1. Install [Rust toolchain](https://www.rust-lang.org/tools/install)
2. git clone https://github.com/tiann/eadb
3. cargo build

### Cargo

If you have Rust toolchain installed, you can install it with cargo:

```sh
cargo install eadb
```

## Credits

All my credits to [adeb](https://github.com/joelagnel/adeb)! eadb is just a rewritten for adeb.

## Contact

twsxtd#gmail.com
