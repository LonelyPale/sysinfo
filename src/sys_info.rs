use std::collections::HashMap;
use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind, Components, Disks};
use colored::{Color, Colorize};

use crate::common::PrettySize;
use crate::disk::disk_info;
use crate::table::{Table, Column};

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

#[derive(Debug)]
pub struct DiskInfoStyle {
    pub kind: InfoStyle,
    pub name: InfoStyle,
    pub file_system: InfoStyle,
    pub mount_point: InfoStyle,
    pub total: InfoStyle,
    pub used: InfoStyle,
    pub free: InfoStyle,
    pub available: InfoStyle,
    pub usage_rate: InfoStyle,
    pub is_removable: InfoStyle,
}

#[derive(Debug)]
pub struct InfoStyle {
    pub name: String,
    pub width: usize,
    // pub display: bool,
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
        Self::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()))
    }

    pub fn new_swap() -> Self {
        Self::new_with_specifics(RefreshKind::new().with_memory(MemoryRefreshKind::new().with_swap()))
    }

    /// 打印全部信息
    pub fn print_all(&mut self, no_color: bool) {
        self.print_system(no_color);
        self.print_cpu(no_color);
        self.print_memory(no_color);
        self.print_swap(no_color);
        self.print_disk(no_color, true, true);

        // Components temperature:
        let components = Components::new_with_refreshed_list();
        println!("=> components:");
        for component in &components {
            println!("{component:?}");
        }
    }

    /// 打印系统信息 Display system information
    pub fn print_system(&self, no_color: bool) {
        let name = System::name().unwrap_or_default();
        let kernel_version = System::kernel_version().unwrap_or_default();
        let os_version = System::os_version().unwrap_or_default();
        let host_name = System::host_name().unwrap_or_default();

        if no_color {
            println!("{} NAME:           {}", "System", name);
            println!("{} kernel version: {}", "System", kernel_version);
            println!("{} OS version:     {}", "System", os_version);
            println!("{} host NAME:      {}", "System", host_name);
        } else {
            println!("{} NAME:           {}", "System".red(), name.blue());
            println!("{} kernel version: {}", "System".red(), kernel_version.cyan());
            println!("{} OS version:     {}", "System".red(), os_version.green());
            println!("{} host NAME:      {}", "System".red(), host_name.purple());
        }

        println!()
    }

    /// 打印CPU信息
    pub fn print_cpu(&mut self, no_color: bool) {
        // Sleeping to let time for the system to run for long
        // enough to have useful information.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        self.system.refresh_cpu(); // Refreshing CPU information.

        let info = self.system.global_cpu_info();
        let core = self.system.physical_core_count();
        let cpus = self.system.cpus();
        let cpu_usage = format!("{:.2}%", info.cpu_usage());
        let cpu_core = format!("{}", core.unwrap_or_default());
        let cpu_thread = format!("{}", cpus.len());

        if no_color {
            println!("{} UsedPercent: {}, Core: {}, Thread: {}", "Cpu", cpu_usage, cpu_core, cpu_thread);
        } else {
            println!("{} UsedPercent: {}, Core: {}, Thread: {}", "Cpu".red(), cpu_usage.blue(), cpu_core.cyan(), cpu_thread.green());
        }

        for cpu in cpus {
            let cpu_usage = format!("{:.2}%", cpu.cpu_usage());
            let frequency = format!("{}", cpu.frequency());
            let name = cpu.name();
            let vendor_id = cpu.vendor_id();
            let brand = cpu.brand();
            if no_color {
                println!("{} {} {} {} {}", name, cpu_usage, frequency, vendor_id, brand);
            } else {
                println!("{} {} {} {} {}", name.yellow(), cpu_usage.blue(), frequency.cyan(), vendor_id.green(), brand.purple());
            }
        }

        println!()
    }

    /// 打印内存信息
    pub fn print_memory(&mut self, no_color: bool) {
        // 通常，“FREE 空闲”内存是指未分配的内存，而“AVAILABLE 可用”内存是指可供（重新）使用的内存。
        // ⚠️ Windows 和 FreeBSD 不报告“可用”内存，因此 free_memory 与 available_memory 的值相同。

        self.system.refresh_memory_specifics(MemoryRefreshKind::new().with_ram());

        let total = self.system.total_memory().pretty_size();
        let used = self.system.used_memory().pretty_size();
        let free = self.system.free_memory().pretty_size();
        let available = self.system.available_memory().pretty_size();
        let used_percent = self.system.used_memory() as f64 / self.system.total_memory() as f64 * 100.0;
        let used_percent = format!("{:.2}%", used_percent);

        if no_color {
            println!("{} Total: {}, Used: {}, Free: {}, Available: {}, UsedPercent: {}", "Memory", total, used, free, available, used_percent);
        } else {
            println!("{} Total: {}, Used: {}, Free: {}, Available: {}, UsedPercent: {}", "Memory".red(), total.blue(), used.cyan(), free.green(), available.yellow(), used_percent.purple());
        }

        println!()
    }

    /// 打印交换分区信息
    pub fn print_swap(&mut self, no_color: bool) {
        self.system.refresh_memory_specifics(MemoryRefreshKind::new().with_swap());

        let total = self.system.total_swap().pretty_size();
        let used = self.system.used_swap().pretty_size();
        let free = self.system.free_swap().pretty_size();
        let used_percent = self.system.used_swap() as f64 / self.system.total_swap() as f64 * 100.0;
        let used_percent = format!("{:.2}%", used_percent);

        if no_color {
            println!("{} Total: {}, Used: {}, Free: {}, UsedPercent: {}", "Swap", total, used, free, used_percent);
        } else {
            println!("{} Total: {}, Used: {}, Free: {}, UsedPercent: {}", "Swap".red(), total.blue(), used.cyan(), free.green(), used_percent.purple());
        }

        println!()
    }

    pub fn print_disk(&self, no_color: bool, all: bool, total: bool) {
        let columns = vec![
            Column {
                title: "Device".to_string(),
                key: "name".to_string(),
                color: Some(Color::Red),
                ..Column::default()
            },
            Column {
                title: "Kind".to_string(),
                key: "kind".to_string(),
                color: Some(Color::Green),
                ..Column::default()
            },
            Column {
                title: "Filesystem".to_string(),
                key: "file_system".to_string(),
                color: Some(Color::Yellow),
                ..Column::default()
            },
            Column {
                title: "Total".to_string(),
                key: "total_space".to_string(),
                right_align: true,
                color: Some(Color::Blue),
                ..Column::default()
            },
            Column {
                title: "Used".to_string(),
                key: "used_space".to_string(),
                right_align: true,
                color: Some(Color::Magenta),
                ..Column::default()
            },
            Column {
                title: "Free".to_string(),
                key: "free_space".to_string(),
                right_align: true,
                color: Some(Color::Cyan),
                ..Column::default()
            },
            Column {
                title: "Avail".to_string(),
                key: "available_space".to_string(),
                hidden: all,
                right_align: true,
                color: Some(Color::Blue),
                ..Column::default()
            },
            Column {
                title: "Use%".to_string(),
                key: "usage_rate".to_string(),
                right_align: true,
                color: Some(Color::Magenta),
                ..Column::default()
            },
            Column {
                title: "MountPoint".to_string(),
                key: "mount_point".to_string(),
                color: Some(Color::BrightRed),
                ..Column::default()
            },
            Column {
                title: "Removable".to_string(),
                key: "is_removable".to_string(),
                hidden: all,
                ..Column::default()
            },
        ];

        let mut data = Vec::new();
        let disks = Disks::new_with_refreshed_list();
        let mut total_total = 0;
        let mut total_used = 0;
        let mut total_free = 0;
        let mut total_avail = 0;
        let mut total_usage = 0.;
        for disk in &disks {
            let kind: String = disk.kind().to_string();
            let name: String = disk.name().to_str().unwrap_or_default().to_string();
            let file_system: String = disk.file_system().to_str().unwrap_or_default().to_string();
            let mount_point: String = disk.mount_point().to_str().unwrap_or_default().to_string();
            let total_space: String = disk.total_space().pretty_size();
            let available_space: String = disk.available_space().pretty_size();
            let is_removable: String = disk.is_removable().to_string();

            let mut free_size: u64 = 0;
            let disk_info_result = disk_info(&mount_point);
            match disk_info_result {
                Ok(res) => { free_size = res.f_bfree * res.f_bsize }
                Err(err) => { eprintln!("print_disk disk_info error: {}", err.red()) }
            }
            let free_space: String = free_size.pretty_size();

            let used_size = disk.total_space() - free_size;
            let used_space = used_size.pretty_size();

            let usage_rate_num = used_size as f64 / disk.total_space() as f64 * 100.;
            let usage_rate = format!("{usage_rate_num:.2}%", );
            // let usage_rate = format!("{usage_rate_num:.0}%", );

            data.push(HashMap::from([
                ("name".to_string(), name),
                ("kind".to_string(), kind),
                ("file_system".to_string(), file_system),
                ("total_space".to_string(), total_space),
                ("used_space".to_string(), used_space),
                ("free_space".to_string(), free_space),
                ("available_space".to_string(), available_space),
                ("usage_rate".to_string(), usage_rate),
                ("mount_point".to_string(), mount_point),
                ("is_removable".to_string(), is_removable),
            ]));

            if total {
                total_total += disk.total_space();
                total_used += used_size;
                total_free += free_size;
                total_avail += disk.available_space();
                total_usage += usage_rate_num;
            }
        }

        if total {
            total_usage = total_usage / disks.len() as f64;
            let total_usage_rate = format!("{total_usage:.2}%", );
            data.push(HashMap::from([
                ("name".to_string(), "total".to_string()),
                ("kind".to_string(), "-".to_string()),
                ("file_system".to_string(), "-".to_string()),
                ("total_space".to_string(), total_total.pretty_size()),
                ("used_space".to_string(), total_used.pretty_size()),
                ("free_space".to_string(), total_free.pretty_size()),
                ("available_space".to_string(), total_avail.pretty_size()),
                ("usage_rate".to_string(), total_usage_rate),
                ("mount_point".to_string(), "-".to_string()),
                ("is_removable".to_string(), "-".to_string()),
            ]));
        }

        let table = Table::new(columns, data);
        println!("{}", table);
        println!();
    }
}

fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}

#[test]
fn test_type() {
    let a = 42;
    println!("a={:?} type={}", a, type_of(a));

    let a = "abc";
    println!("a={:?} type={}", a, type_of(a));

    let a = String::from("测试字符串");
    println!("a={:?} type={}", a, type_of(&a));
}

#[test]
fn test_print_all() {
    SysInfo::new_all().print_all(false);
}

#[test]
fn test_print_system() {
    SysInfo::new().print_system(false);
}

#[test]
fn test_print_cpu() {
    SysInfo::new_cpu().print_cpu(false);
}

#[test]
fn test_print_memory() {
    SysInfo::new_memory().print_memory(false);
}

#[test]
fn test_print_swap() {
    SysInfo::new_swap().print_swap(false);
}

#[test]
fn test_print_disk() {
    SysInfo::new().print_disk(false);
}
