mod common;
mod sys_info;
mod disk;

use std::fmt::Debug;
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
            SysInfo::new_all().print_all(args.no_color);
        }
        // _ => {
        //     println!("testing...");
        // }
    }
}
