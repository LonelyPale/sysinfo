# sysinfo
Command line program to display system information

```shell
git clone https://github.com/LonelyPale/sysinfo-cli

```

```shell
cargo add clap
cargo add clap --features derive
cargo add colored
cargo add sysinfo
cargo add libc

cargo add psutil
cargo remove psutil

```

# Local Build 本地编译
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo init --bin .

#debug
cargo build
cargo build --bin sysinfo

#release
cargo build --release
cargo build --release --bin sysinfo
cargo build --release --target-dir target/darwin

```

# Cross Build 交叉编译
```shell
rustup target list
rustup target add x86_64-unknown-linux-gnu

cargo build --target x86_64-unknown-linux-gnu

TARGET_CC=x86_64-unknown-linux-gnu cargo build --release --target x86_64-unknown-linux-gnu

cargo install cargo-zigbuild
cargo zigbuild --release --target x86_64-unknown-linux-gnu

```

# Docker Build 容器编译

```shell
docker pull rust:1.76.0-buster

docker run -it --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.76.0-buster cargo build --release
docker run -it --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust:1.76.0-buster cargo build --release --target-dir target/linux

md5sum sysinfo*
sha256sum sysinfo*
shasum -a 256 sysinfo*
```

# Install

```shell
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/LonelyPale/sysinfo-cli/main/bin/install.py | python3

```
