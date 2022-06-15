# eadb - eBPF Android Debug Bridge 

eadb provides a powerful Linux shell environment where one can run [BCC](https://github.com/iovisor/bcc) / [bpftrace](https://github.com/iovisor/bpftrace) / [bpftool](https://github.com/libbpf/bpftool) on Android device.

## Usage

Install the eadb first, please refer [Install](https://github.com/tiann/eadb#install)

### Prepare eadb environment on Android device

eadb support two mode to connect the device:

- adb
- ssh

Both of them need the root privilege. 

If you want to use `adb` mode, `adb root` is required but `adb root` is disabled in user build, if you doesn't have a userdebug / eng build, you can try `ssh` mode.

If you use `ssh` mode, it is recommended to install [Magisk](https://github.com/topjohnwu/Magisk) to Root your device and install [MagiskSSH](https://gitlab.com/d4rcm4rc/MagiskSSH_releases) to enable ssh.

#### Download it from github

When you can use adb or ssh to connect your device, you can prepare the eadb environment from github by this command:

```
eadb --ssh root@ip prepare
```

It would download a rootfs from [Release page](https://github.com/tiann/eadb/releases) and push it to your device, then do some mounts and chroot in to the environment.

#### Use an existing archive

```
eadb --ssh root@ip prepare -a path/to/archive
```

### Enter the environment

```
eadb --ssh root@ip shell
```

Then you will enter the eadb environment, you can use `apt update` to update the sources and install softwares(such as clang,llvm,bpftrace) by yourself, you install Rust / Golang or gcc to delelop on this device!

### Build the environment by yourself

Only Ubuntu / Debian is supported to build the system image running on Android, you can use docker or podman on macOS and WSL on Windows.

Install `qemu-user-static` and `debootstrap` first:

```
apt update && apt install qemu-user-static debootstrap
```

And then build the eadb (root is required):

```
sudo eadb build
```

After the build, you will get a `deb.tar.gz` in your working directory. you can use this image as your environment:

```
eadb --ssh root@ip prepare -a deb.tar.gz
```

## Install

### Binary

Download binaries in [Release page](https://github.com/tiann/eadb/releases)

### Build from source

1. Install [Rust toolchain](https://www.rust-lang.org/tools/install)
2. git clone https://github.com/tiann/eadb
3. cargo build

## Credits

All my credits to [adeb](https://github.com/joelagnel/adeb)! eadb is just a rewritten for adeb.

## Contact

twsxtd#gmail.com
