# sysinfo
Command line program to display system information

```shell
git clone https://github.com/LonelyPale/sysinfo-cli

```

```shell
cargo init --bin .

#debug
cargo build
cargo build --bin sysinfo

#release
cargo build --release
cargo build --release --bin sysinfo


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

# Install

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/LonelyPale/sysinfo-cli/main/bin/install.py | sh

```
