mod common;
mod sys_info;
mod disk;

use std::fmt::Debug;
use std::mem;
use clap::{Parser, Subcommand, ArgAction};

use sys_info::{SysInfo};

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "sysinfo", bin_name = "sysinfo")]
#[command(version = "0.1.0", long_version = "0.1.0.888")]
#[command(about = "Display system information CLI", long_about = None)]
#[command(disable_version_flag = true)] //禁用version
// #[command(disable_help_flag = true)] //禁用help
// #[command(next_line_help = true)] //一条记录分两行显示
// #[command(ignore_errors = true)] //忽略error
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>, //Option使子命令不是必须的

    /// Disable color
    #[arg(short = 'c', long)]
    no_color: bool,

    //[custom_help_flag](https://github.com/clap-rs/clap/blob/v4.4.18/tests/derive/help.rs#L430)
    //[custom_version_flag](https://github.com/clap-rs/clap/blob/v4.4.18/tests/derive/help.rs#L446)
    /// Print version
    #[arg(short = 'v', long = "version", action = ArgAction::Version, value_parser = clap::value_parser ! (bool))]
    version: (),
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Print system info
    System {},

    /// Print cpu info
    Cpu {},

    /// Print memory info
    Memory {},

    /// Print swap info
    Swap {},

    /// Print disk info
    Disk {},
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::System {}) => {
            SysInfo::new().print_system(args.no_color);
        }
        Some(Commands::Cpu {}) => {
            SysInfo::new_cpu().print_cpu(args.no_color);
        }
        Some(Commands::Memory {}) => {
            SysInfo::new_memory().print_memory(args.no_color);
        }
        Some(Commands::Swap {}) => {
            SysInfo::new_swap().print_swap(args.no_color);
        }
        Some(Commands::Disk {}) => {
            SysInfo::new().print_disk(args.no_color);
        }
        None => {
            test();
            println!();
            // SysInfo::new_all().print_all(args.no_color);
        }
        // _ => {
        //     println!("{}", "testing...".yellow().bold());
        // }
    }
}


#[cfg(all(unix, not(feature = "unknown-ci")))]
macro_rules! retry_eintr {
    (set_to_0 => $($t:tt)+) => {{
        println!("111");
        let errno = crate::unix::libc_errno();
        if !errno.is_null() {
            *errno = 0;
        }
        retry_eintr!($($t)+)
    }};
    ($errno_value:ident => $($t:tt)+) => {{
        println!("222");
        loop {
            let ret = $($t)+;
            if ret < 0 {
                let tmp = std::io::Error::last_os_error();
                if tmp.kind() == std::io::ErrorKind::Interrupted {
                    continue;
                }
                $errno_value = tmp.raw_os_error().unwrap_or(0);
            }
            break ret;
        }
    }};
    ($($t:tt)+) => {{
        println!("333");
        loop {
            let ret = $($t)+;
            if ret < 0 && std::io::Error::last_os_error().kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            break ret;
        }
    }};
}


extern crate libc;

use libc::{statvfs, c_char};

fn test() {
    let path = std::path::Path::new("/tmp4");
    let mount_point_cpath = to_cpath(path);

    unsafe {
        let mut stat: statvfs = mem::zeroed();
        let res = retry_eintr!(statvfs(mount_point_cpath.as_ptr() as *const _, &mut stat));

        if res == 0 {
            println!("Filesystem information:");
            println!("f_bsize: {}", stat.f_bsize);
            println!("f_frsize: {}", stat.f_frsize);
            println!("f_blocks: {}", stat.f_blocks);
            println!("f_bfree: {}", stat.f_bfree);
            println!("f_bavail: {}", stat.f_bavail);
            println!("f_files: {}", stat.f_files);
            println!("f_filefree: {}", stat.f_ffree);
            println!("f_favail: {}", stat.f_favail);
            // println!("f_flag: {:#x}", stat.f_flag);
            // println!("f_namemax: {}", stat.f_namemax);
        } else {
            eprintln!("Failed to get filesystem information");
        }
    };

}

pub(crate) fn to_cpath(path: &std::path::Path) -> Vec<u8> {
    use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

    let path_os: &OsStr = path.as_ref();
    let mut cpath = path_os.as_bytes().to_vec();
    cpath.push(0);
    cpath
}

