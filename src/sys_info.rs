use colored::{Color, Colorize, Style, Styles};
use std::collections::HashMap;
use sysinfo::{CpuRefreshKind, Disks, MemoryRefreshKind, RefreshKind, System};

use crate::disk::disk_info;
use crate::common::{BaseSize, BlockSize, PrettySize};
use crate::table::{Column, CombineString, RenderArgs, Table};
use crate::Commands;

#[derive(Debug)]
pub struct SysInfo {
    system: System,
}

#[derive(Debug)]
pub struct DiskInfo {
    pub kind: String,
    pub name: String,
    pub file_system: String,
    pub mount_point: String,
    pub total: String,
    pub used: String,
    pub free: String,
    pub available: String,
    pub usage_rate: String,
    pub is_removable: String,
}

impl SysInfo {
    fn new_with_specifics(refreshes: RefreshKind) -> Self {
        Self {
            system: System::new_with_specifics(refreshes),
        }
    }

    pub fn new() -> Self {
        Self::new_with_specifics(RefreshKind::new())
    }

    pub fn new_all() -> Self {
        Self::new_with_specifics(RefreshKind::new().without_processes())
    }

    pub fn new_cpu() -> Self {
        Self::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()))
    }

    pub fn new_memory() -> Self {
        Self::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::everything()))
    }

    /// 打印全部信息
    pub fn print_all(&mut self) {
        self.print_system();
        println!();
        println!();

        self.print_cpu(true);
        println!();
        println!();

        self.print_memory();
        println!();
        println!();

        let cmd = Commands::Disk {
            all: true,
            sort: "".to_string(), //"MountPoint".to_string()
            exclude: "".to_string(), //"Type:overlay".to_string()
            total: true,
            human_readable: false,
            si: false,
            block_size: "".to_string(),
        };
        self.print_disk(cmd);

        // Components temperature:
        // let components = sysinfo::Components::new_with_refreshed_list();
        // println!("=> components:");
        // for component in &components {
        //     println!("{component:?}");
        // }
    }

    /// 打印系统信息 Display system information
    pub fn print_system(&self) {
        let os_name = System::name().unwrap_or_default();
        let os_version = System::os_version().unwrap_or_default();
        let kernel_version = System::kernel_version().unwrap_or_default();
        let host_name = System::host_name().unwrap_or_default();

        let width = 15;
        println!("{:width$} {}", "OS Name:".color(Color::Red), os_name.color(Color::Green));
        println!("{:width$} {}", "OS Version:".color(Color::Red), os_version.color(Color::Blue));
        println!("{:width$} {}", "Kernel Version:".color(Color::Red), kernel_version.color(Color::Yellow));
        println!("{:width$} {}", "Hostname:".color(Color::Red), host_name.color(Color::Magenta));
    }

    /// 打印CPU信息
    pub fn print_cpu(&mut self, details: bool) {
        let columns = vec![
            Column {
                title: "".to_string(),
                key: "title".to_string(),
                color: Some(Color::Red),
                style: Style::default() | Styles::Bold,
                ..Column::default()
            },
            Column {
                title: "Use%".to_string(),
                key: "cpu_usage".to_string(),
                right_align: true,
                color: Some(Color::Green),
                ..Column::default()
            },
            Column {
                title: "Core".to_string(),
                key: "cpu_core".to_string(),
                right_align: true,
                color: Some(Color::Yellow),
                ..Column::default()
            },
            Column {
                title: "Thread".to_string(),
                key: "cpu_thread".to_string(),
                right_align: true,
                color: Some(Color::Blue),
                ..Column::default()
            },
        ];

        // Sleeping to let time for the system to run for long
        // enough to have useful information.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.system.refresh_cpu(); // Refreshing CPU information.

        //全局 global
        let info = self.system.global_cpu_info();
        let core = self.system.physical_core_count();
        let cpus = self.system.cpus();
        let cpu_usage = format!("{:.2}%", info.cpu_usage());
        let cpu_core = format!("{}", core.unwrap_or_default());
        let cpu_thread = format!("{}", cpus.len());

        let mut data = Vec::new();
        data.push(HashMap::from([
            ("title".to_string(), "CPU:".to_string()),
            ("cpu_usage".to_string(), cpu_usage),
            ("cpu_core".to_string(), cpu_core),
            ("cpu_thread".to_string(), cpu_thread),
        ]));

        let table = Table::new(columns, data, HashMap::new());
        println!("{}", table);

        if details {
            //明细 details
            let columns_details = vec![
                Column {
                    title: "Name".to_string(),
                    key: "name".to_string(),
                    color: Some(Color::Red),
                    ..Column::default()
                },
                Column {
                    title: "Use%".to_string(),
                    key: "cpu_usage".to_string(),
                    right_align: true,
                    color: Some(Color::Green),
                    ..Column::default()
                },
                Column {
                    title: "Frequency".to_string(),
                    key: "frequency".to_string(),
                    right_align: true,
                    color: Some(Color::Yellow),
                    ..Column::default()
                },
                Column {
                    title: "VendorID".to_string(),
                    key: "vendor_id".to_string(),
                    color: Some(Color::Blue),
                    ..Column::default()
                },
                Column {
                    title: "Brand".to_string(),
                    key: "brand".to_string(),
                    color: Some(Color::Magenta),
                    ..Column::default()
                },
            ];

            let mut data_details = Vec::new();
            for cpu in cpus {
                let name = cpu.name();
                let cpu_usage = format!("{:.2}%", cpu.cpu_usage());
                let frequency = format!("{}", cpu.frequency());
                let vendor_id = cpu.vendor_id();
                let brand = cpu.brand();

                data_details.push(HashMap::from([
                    ("name".to_string(), name.to_string()),
                    ("cpu_usage".to_string(), cpu_usage),
                    ("frequency".to_string(), frequency),
                    ("vendor_id".to_string(), vendor_id.to_string()),
                    ("brand".to_string(), brand.to_string()),
                ]));
            }

            let table_details = Table::new(columns_details, data_details, HashMap::new());
            println!();
            println!();
            println!("{}", table_details);
        }
    }

    /// 打印内存、交换分区信息
    pub fn print_memory(&mut self) {
        let columns = vec![
            Column {
                title: "".to_string(),
                key: "title".to_string(),
                color: Some(Color::Red),
                style: Style::default() | Styles::Bold,
                ..Column::default()
            },
            Column {
                title: "Total".to_string(),
                key: "total".to_string(),
                right_align: true,
                color: Some(Color::Green),
                ..Column::default()
            },
            Column {
                title: "Used".to_string(),
                key: "used".to_string(),
                right_align: true,
                color: Some(Color::Yellow),
                ..Column::default()
            },
            Column {
                title: "Free".to_string(),
                key: "free".to_string(),
                right_align: true,
                color: Some(Color::Blue),
                ..Column::default()
            },
            Column {
                title: "Avail".to_string(),
                key: "available".to_string(),
                right_align: true,
                color: Some(Color::Magenta),
                ..Column::default()
            },
            Column {
                title: "Use%".to_string(),
                key: "used_percent".to_string(),
                right_align: true,
                color: Some(Color::Cyan),
                ..Column::default()
            },
        ];

        // memory
        // 通常，“FREE 空闲”内存是指未分配的内存，而“AVAILABLE 可用”内存是指可供（重新）使用的内存。
        // ⚠️ Windows 和 FreeBSD 不报告“可用”内存，因此 free_memory 与 available_memory 的值相同。

        self.system.refresh_memory_specifics(MemoryRefreshKind::new().with_ram());

        let total = self.system.total_memory().pretty_size();
        let used = self.system.used_memory().pretty_size();
        let free = self.system.free_memory().pretty_size();
        let available = self.system.available_memory().pretty_size();
        let used_percent = self.system.used_memory() as f64 / self.system.total_memory() as f64 * 100.0;
        let used_percent = format!("{:.2}%", used_percent);

        let mut data = Vec::new();
        data.push(HashMap::from([
            ("title".to_string(), "Memory:".to_string()),
            ("total".to_string(), total),
            ("used".to_string(), used),
            ("free".to_string(), free),
            ("available".to_string(), available),
            ("used_percent".to_string(), used_percent),
        ]));

        // swap
        self.system.refresh_memory_specifics(MemoryRefreshKind::new().with_swap());

        let total = self.system.total_swap().pretty_size();
        let used = self.system.used_swap().pretty_size();
        let free = self.system.free_swap().pretty_size();
        let used_percent = self.system.used_swap() as f64 / self.system.total_swap() as f64 * 100.0;
        let used_percent = format!("{:.2}%", used_percent);

        data.push(HashMap::from([
            ("title".to_string(), "Swap:".to_string()),
            ("total".to_string(), total),
            ("used".to_string(), used),
            ("free".to_string(), free),
            ("available".to_string(), "".to_string()),
            ("used_percent".to_string(), used_percent),
        ]));

        let table = Table::new(columns, data, HashMap::new());
        println!("{}", table);
    }

    pub fn print_disk(&self, cmd: Commands) {
        let Commands::Disk { all, sort, exclude, total, human_readable, si, block_size, .. } = cmd else { todo!() };

        let mut base: BaseSize = BaseSize::Size1024;
        let mut block: BlockSize = BlockSize::Auto;
        if human_readable {
            base = BaseSize::Size1024;
            block = BlockSize::Auto;
        } else if si {
            base = BaseSize::Size1000;
            block = BlockSize::Auto;
        }

        if block_size.len() > 0 {
            let result: Result<BlockSize, _> = block_size.parse();
            block = match result {
                Ok(val) => { val }
                Err(err) => {
                    eprintln!("{err}: {block_size}");
                    return;
                }
            }
        }

        // let render = |args: RenderArgs| -> CombineString {//closure-error: 无法解决
        fn render(args: RenderArgs) -> CombineString {
            let RenderArgs { value, column, record_index, data, custom, .. } = args;
            let mut total = false;
            if let Some(val) = custom.get("total") {
                if val == "true" {
                    total = true;
                }
            }
            let last = data.len() - 1;
            if total && record_index == last {
                let mut value = match value {
                    CombineString::AsStr(val) => val.normal(),
                    CombineString::AsString(val) => val.normal(),
                    CombineString::AsColoredString(val) => val,
                };

                if value.is_empty() {
                    return CombineString::AsColoredString(value);
                }

                if value.fgcolor.is_none() {
                    value.fgcolor = column.color;
                }

                value.style |= Styles::Bold;

                CombineString::AsColoredString(value)
            } else {
                value
            }
        }

        let columns = vec![
            Column {
                title: "Device".to_string(),
                key: "name".to_string(),
                color: Some(Color::Red),
                render: Some(render),
                ..Column::default()
            },
            Column {
                title: "Type".to_string(),
                key: "file_system".to_string(),
                color: Some(Color::Green),
                render: Some(render),
                ..Column::default()
            },
            Column {
                title: "Kind".to_string(),
                key: "kind".to_string(),
                color: Some(Color::Yellow),
                render: Some(render),
                ..Column::default()
            },
            Column {
                title: "Total".to_string(),
                key: "total_space".to_string(),
                right_align: true,
                color: Some(Color::Blue),
                render: Some(render),
                ..Column::default()
            },
            Column {
                title: "Used".to_string(),
                key: "used_space".to_string(),
                right_align: true,
                color: Some(Color::Magenta),
                render: Some(render),
                ..Column::default()
            },
            Column {
                title: "Free".to_string(),
                key: "free_space".to_string(),
                right_align: true,
                color: Some(Color::Cyan),
                render: Some(render),
                ..Column::default()
            },
            Column {
                title: "Avail".to_string(),
                key: "available_space".to_string(),
                hidden: !all,
                right_align: true,
                color: Some(Color::BrightRed),
                render: Some(render),
                ..Column::default()
            },
            Column {
                title: "Use%".to_string(),
                key: "usage_rate".to_string(),
                right_align: true,
                color: Some(Color::BrightGreen),
                render: Some(render),
                ..Column::default()
            },
            Column {
                title: "MountPoint".to_string(),
                key: "mount_point".to_string(),
                color: Some(Color::BrightYellow),
                render: Some(render),
                ..Column::default()
            },
            Column {
                title: "Removable".to_string(),
                key: "is_removable".to_string(),
                color: Some(Color::BrightBlue),
                hidden: !all,
                render: Some(render),
                ..Column::default()
            },
        ];

        // FIELD:VALUE1,VALUE2
        let mut exclude_key: &str = "";
        let mut exclude_vals: Vec<&str> = Vec::new();
        if exclude.len() > 0 {
            let parts: Vec<&str> = exclude.split(":").collect();
            if parts.len() != 2 {
                let err = format!("Invalid exclude: {}", exclude).red();
                eprintln!("{}", err);
                return;
            }

            let title = parts[0];
            for col in &columns {
                if title == col.title {
                    exclude_key = &col.key;
                    break;
                }
            }
            if exclude_key.len() == 0 {
                let err = format!("Invalid exclude: {}", exclude).red();
                eprintln!("{}", err);
                return;
            }

            let values = parts[1];
            exclude_vals = values.split(",").collect();
            if exclude_vals.len() == 0 {
                let err = format!("Invalid exclude: {}", exclude).red();
                eprintln!("{}", err);
                return;
            }
        }

        let mut data = Vec::new();
        let disks = Disks::new_with_refreshed_list();
        let mut total_total = 0;
        let mut total_used = 0;
        let mut total_free = 0;
        let mut total_avail = 0;
        for disk in &disks {
            let kind: String = disk.kind().to_string();
            let name: String = disk.name().to_str().unwrap_or_default().to_string();
            let file_system: String = disk.file_system().to_str().unwrap_or_default().to_string();
            let mount_point: String = disk.mount_point().to_str().unwrap_or_default().to_string();
            let total_space: String = disk.total_space().pretty_size_with(base, block);
            let available_space: String = disk.available_space().pretty_size_with(base, block);
            let is_removable: String = disk.is_removable().to_string();

            let mut free_size: u64 = 0;
            let disk_info_result = disk_info(&mount_point);
            match disk_info_result {
                Ok(res) => free_size = res.f_bfree * res.f_bsize,
                Err(err) => {
                    eprintln!("print_disk disk_info error: {}", err.red())
                }
            }
            let free_space: String = free_size.pretty_size_with(base, block);

            let used_size = disk.total_space() - free_size;
            let used_space = used_size.pretty_size_with(base, block);

            let usage_rate_num = used_size as f64 / disk.total_space() as f64 * 100.;
            let usage_rate = format!("{usage_rate_num:.2}%");

            let row = HashMap::from([
                ("name".to_string(), name),
                ("file_system".to_string(), file_system),
                ("kind".to_string(), kind),
                ("total_space".to_string(), total_space),
                ("total".to_string(), disk.total_space().to_string()), //额外增加，仅排序用
                ("used_space".to_string(), used_space),
                ("used".to_string(), used_size.to_string()), //额外增加，仅排序用
                ("free_space".to_string(), free_space),
                ("free".to_string(), free_size.to_string()), //额外增加，仅排序用
                ("available_space".to_string(), available_space),
                ("available".to_string(), disk.available_space().to_string()), //额外增加，仅排序用
                ("usage_rate".to_string(), usage_rate),
                ("mount_point".to_string(), mount_point),
                ("is_removable".to_string(), is_removable),
            ]);

            let mut exclude_flag = false;
            if exclude_key.len() > 0 && exclude_vals.len() > 0 {
                exclude_flag = exclude_record_disk(exclude_key, &exclude_vals, &row);
            }

            if !exclude_flag {
                data.push(row);
                if total {
                    total_total += disk.total_space();
                    total_used += used_size;
                    total_free += free_size;
                    total_avail += disk.available_space();
                }
            }
        }

        if sort.len() > 0 {
            let mut key = "";
            for col in &columns {
                if sort == col.title {
                    key = &col.key;
                    break;
                }
            }

            let last = key.len() - "_space".len();
            if key.len() > 0 {
                data.sort_by(|a, b| {
                    let mut key = key;
                    if key.ends_with("_space") {
                        key = &key[..last]
                    }
                    let empty = &"".to_string();
                    let val_a = a.get(key).unwrap_or(empty);
                    let val_b = b.get(key).unwrap_or(empty);
                    if key == "total" || key == "used" || key == "free" || key == "available" {
                        let u64_a: u64 = val_a.parse().unwrap_or_default();
                        let u64_b: u64 = val_b.parse().unwrap_or_default();
                        u64_a.cmp(&u64_b)
                    } else if key == "usage_rate" {
                        let str_a = val_a.get(..val_a.len() - 1).unwrap_or_default();
                        let str_b = val_b.get(..val_b.len() - 1).unwrap_or_default();
                        let f64_a: f64 = str_a.parse().unwrap_or_default();
                        let f64_b: f64 = str_b.parse().unwrap_or_default();
                        f64_a.total_cmp(&f64_b)
                    } else {
                        val_a.cmp(val_b)
                    }
                });
            }
        }

        if total {
            let total_usage = total_used as f64 / total_total as f64 * 100.;
            let total_usage_rate = format!("{total_usage:.2}%");
            data.push(HashMap::from([
                ("name".to_string(), "total".to_string()),
                ("total_space".to_string(), total_total.pretty_size_with(base, block)),
                ("used_space".to_string(), total_used.pretty_size_with(base, block)),
                ("free_space".to_string(), total_free.pretty_size_with(base, block)),
                ("available_space".to_string(), total_avail.pretty_size_with(base, block)),
                ("usage_rate".to_string(), total_usage_rate),
            ]));
        }

        let custom = HashMap::from([
            ("total".to_string(), total.to_string()),
        ]);
        let table = Table::new(columns, data, custom);
        println!("{}", table);
    }
}

fn exclude_record_disk(exclude_key: &str, exclude_vals: &Vec<&str>, row: &HashMap<String, String>) -> bool {
    let val_opt = row.get(exclude_key);
    return if let Some(val) = val_opt {
        for ev in exclude_vals {
            if ev == val {
                return true;
            }
        }
        false
    } else {
        false
    };
}

fn _type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

#[test]
fn test_type() {
    let a = 42;
    println!("a={:?} type={}", a, _type_of(a));

    let a = "abc";
    println!("a={:?} type={}", a, _type_of(a));

    let a = String::from("测试字符串");
    println!("a={:?} type={}", a, _type_of(&a));
}

#[test]
fn test_print_all() {
    SysInfo::new_all().print_all();
}

#[test]
fn test_print_system() {
    SysInfo::new().print_system();
}

#[test]
fn test_print_cpu() {
    SysInfo::new_cpu().print_cpu(false);
}

#[test]
fn test_print_memory() {
    SysInfo::new_memory().print_memory();
}

#[test]
fn test_print_disk() {
    let cmd = Commands::Disk {
        all: true,
        sort: "MountPoint".to_string(),
        exclude: "Type:overlay".to_string(),
        total: true,
        human_readable: false,
        si: false,
        block_size: "".to_string(),
    };
    SysInfo::new().print_disk(cmd);
}

fn _demo_color() {
    println!("demo_color:");
    println!("{}", "Black".black());
    println!("{}", "BrightBlack".bright_black());
    println!("{}", "Red".red());
    println!("{}", "BrightRed".bright_red());
    println!("{}", "Green".green());
    println!("{}", "BrightGreen".bright_green());
    println!("{}", "Yellow".yellow());
    println!("{}", "BrightYellow".bright_yellow());
    println!("{}", "Blue".blue());
    println!("{}", "BrightBlue".bright_blue());
    println!("{}", "Magenta".magenta());
    println!("{}", "BrightMagenta".bright_magenta());
    println!("{}", "Cyan".cyan());
    println!("{}", "BrightCyan".bright_cyan());
    println!("{}", "White".white());
    println!("{}", "BrightWhite".bright_white());
    println!("{}", "TrueColor".truecolor(0, 255, 136));
    println!();
}

fn _demo_style() {
    println!("demo_style:");
    println!("{}", "Clear".clear());
    println!("{}", "Bold".bold());
    println!("{}", "Dimmed".dimmed());
    println!("{}", "Underline".underline());
    println!("{}", "Reversed".reversed());
    println!("{}", "Italic".italic());
    println!("{}", "Blink".blink());
    println!("{}", "Hidden".hidden());
    println!("{}", "Strikethrough".strikethrough());
    println!();
}

fn _demo_control() {
    println!("demo_control:");
    // this will be yellow if your environment allow it
    println!("{}", "some warning-1".yellow());
    // now , this will be always yellow
    colored::control::set_override(true);
    println!("{}", "some warning-2".yellow());
    println!("{}", "some warning-20");
    println!("{}", "some warning-21".red());
    // now, this will be never yellow
    colored::control::set_override(false);
    println!("{}", "some warning-3".yellow());
    println!("{}", "some warning-30".red());
    println!("{}", "some warning-31".bold().underline());
    // let the environment decide again
    colored::control::unset_override();
    println!("{}", "some warning-4".yellow());
    println!("{}", "some warning-40".red());
    println!("{}", "some warning-41".bold().underline());
    println!();
}

#[test]
fn test_demo() {
    _demo_color();
    _demo_style();
    _demo_control();
}
