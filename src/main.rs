mod common;
mod disk;
mod sys_info;
mod table;

use std::fmt::Debug;
use clap::{ArgAction, Parser, Subcommand};

use sys_info::SysInfo;

#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "sysinfo", bin_name = "sysinfo")]
#[command(version = "0.1.0", long_version = "0.1.0 (2024-02-22)")]
#[command(about = "Display system information CLI", long_about = None)]
#[command(after_help = "Additional help information can be found here. (https://github.com/LonelyPale/sysinfo-cli)", after_long_help = None)] //自定义help后输出的内容，使用属性宏clap和command都可以
#[command(disable_version_flag = true)] //禁用version
#[command(disable_help_flag = true)] //禁用help
// #[command(color = ColorChoice::Always)] //启用颜色输出，没有效果
// #[command(author = "Your Name. <email@example.com>")] //作者信息
// #[command(next_line_help = true)] //一条记录分两行显示，参数在第一行，帮助在第二行
// #[command(ignore_errors = true)] //忽略error
// #[clap(after_help = "Additional help information can be found here.")] //自定义help后输出的内容，使用属性宏clap和command都可以
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>, //Option使子命令不是必须的

    /// Disable colorize
    #[arg(short = 'c', long)]
    no_color: bool,

    //[custom_version_flag](https://github.com/clap-rs/clap/blob/v4.4.18/tests/derive/help.rs#L446)
    /// Print version
    #[arg(short = 'v', long = "version", action = ArgAction::Version, value_parser = clap::value_parser ! (bool))]
    version: (),

    //short = 'h',
    //[custom_help_flag](https://github.com/clap-rs/clap/blob/v4.4.18/tests/derive/help.rs#L430)
    //[before_help、before_long_help](https://github.com/clap-rs/clap/blob/v4.4.18/tests/builder/help.rs#L235)
    //[after_help、after_long_help](https://github.com/clap-rs/clap/blob/v4.4.18/tests/builder/help.rs#L259)
    /// Print help
    #[arg(long = "help", action = ArgAction::Help, value_parser = clap::value_parser ! (bool))]
    help: (),
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print system info
    System {},

    /// Print cpu info
    Cpu {
        /// Print cpu details
        #[arg(short, long)]
        details: bool,
    },

    /// Print memory and swap info
    Memory {},

    /// Print disk info
    #[command(after_help = "FIELD is a column to be included.  Valid field names are:
[Device | Type | Kind | Total | Used | Free | Avail | Use% | MountPoint | Removable] (see info page).

The SIZE argument is an integer and optional unit (example: 10K is 10*1024).
Units are K,M,G,T,P,E,Z,Y (powers of 1024) or KB,MB,... (powers of 1000).

Additional help information can be found here. (https://github.com/LonelyPale/sysinfo-cli)
")] //自定义help后输出的内容，使用属性宏clap和command都可以
    Disk {
        /// Print all fields
        #[arg(short, long)]
        all: bool,

        /// Sort by field; see FIELD format below
        ///
        #[arg(short, long, value_name = "FIELD", default_value_t = String::from(""))]
        sort: String,

        /// Limit listing to record not of field and value; see FIELD format below
        #[arg(short, long, value_name = "FIELD:VALUE1,VALUE2", default_value_t = String::from(""))]
        exclude: String,

        /// Generate total value
        #[arg(short, long)]
        total: bool,

        /// Print sizes in powers of 1024 (e.g., 1023M) [default: true]
        #[arg(short = 'h', long)]
        human_readable: bool,

        /// Print sizes in powers of 1000 (e.g., 1.1G) Metric (SI) Prefixes
        #[arg(short = 'H', long)]
        si: bool,

        /// Scale sizes by SIZE before printing them;
        /// e.g., '-BM' prints sizes in units of 1,048,576 bytes;
        /// see SIZE format below
        #[arg(short = 'B', long, value_name = "SIZE", default_value_t = String::from(""))]
        block_size: String,
    },
}

fn main() {
    let args = Cli::parse();

    if args.no_color {
        colored::control::set_override(false);
    }

    match args.command {
        Some(Commands::System {}) => {
            SysInfo::new().print_system();
        }
        Some(Commands::Cpu { details }) => {
            SysInfo::new_cpu().print_cpu(details);
        }
        Some(Commands::Memory {}) => {
            SysInfo::new_memory().print_memory();
        }
        Some(Commands::Disk { .. }) => {
            SysInfo::new().print_disk(args.command.unwrap());
        }
        None => {
            SysInfo::new_all().print_all();
        }
        // _ => {
        //     println!("testing...");
        // }
    }
}
