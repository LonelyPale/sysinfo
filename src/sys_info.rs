use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind, Components, Disks};
use colored::Colorize;

use crate::common::PrettySize;
use crate::disk::disk_info;

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
    pub total_space: String,
    pub free_space: String,
    pub available_space: String,
    pub is_removable: String,
}

#[derive(Debug)]
pub struct DiskInfoStyle {
    pub kind_width_max: usize,
    pub name_width_max: usize,
    pub file_system_width_max: usize,
    pub mount_point_width_max: usize,
    pub total_space_width_max: usize,
    pub free_space_width_max: usize,
    pub available_space_width_max: usize,
    pub is_removable_width_max: usize,
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
        self.print_disk(no_color);

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
            println!("{} name:           {}", "System", name);
            println!("{} kernel version: {}", "System", kernel_version);
            println!("{} OS version:     {}", "System", os_version);
            println!("{} host name:      {}", "System", host_name);
        } else {
            println!("{} name:           {}", "System".red(), name.blue());
            println!("{} kernel version: {}", "System".red(), kernel_version.cyan());
            println!("{} OS version:     {}", "System".red(), os_version.green());
            println!("{} host name:      {}", "System".red(), host_name.purple());
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
        // 通常，“free 空闲”内存是指未分配的内存，而“available 可用”内存是指可供（重新）使用的内存。
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

    pub fn print_disk(&self, no_color: bool) {
        let mut disk_info_vec: Vec<DiskInfo> = Vec::new();
        let mut disk_info_style: DiskInfoStyle = DiskInfoStyle {
            kind_width_max: "kind".len(),
            name_width_max: "name".len(),
            file_system_width_max: "file_system".len(),
            mount_point_width_max: "mount_point".len(),
            total_space_width_max: "total_space".len(),
            free_space_width_max: "free_space".len(),
            available_space_width_max: "available_space".len(),
            is_removable_width_max: "is_removable".len(),
        };

        let disks = Disks::new_with_refreshed_list();
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

            if kind.len() > disk_info_style.kind_width_max {
                disk_info_style.kind_width_max = kind.len();
            }
            if name.len() > disk_info_style.name_width_max {
                disk_info_style.name_width_max = name.len();
            }
            if file_system.len() > disk_info_style.file_system_width_max {
                disk_info_style.file_system_width_max = file_system.len();
            }
            if mount_point.len() > disk_info_style.mount_point_width_max {
                disk_info_style.mount_point_width_max = mount_point.len();
            }
            if total_space.len() > disk_info_style.total_space_width_max {
                disk_info_style.total_space_width_max = total_space.len();
            }
            if free_space.len() > disk_info_style.free_space_width_max {
                disk_info_style.free_space_width_max = free_space.len();
            }
            if available_space.len() > disk_info_style.available_space_width_max {
                disk_info_style.available_space_width_max = available_space.len();
            }
            if is_removable.len() > disk_info_style.is_removable_width_max {
                disk_info_style.is_removable_width_max = is_removable.len();
            }

            let disk_info = DiskInfo {
                kind,
                name,
                file_system,
                mount_point,
                total_space,
                free_space,
                available_space,
                is_removable,
            };
            disk_info_vec.push(disk_info);
        }

        let kind_width_max = disk_info_style.kind_width_max;
        let name_width_max = disk_info_style.name_width_max;
        let file_system_width_max = disk_info_style.file_system_width_max;
        let mount_point_width_max = disk_info_style.mount_point_width_max;
        let total_space_width_max = disk_info_style.total_space_width_max;
        let free_space_width_max = disk_info_style.free_space_width_max;
        let available_space_width_max = disk_info_style.available_space_width_max;
        let is_removable_width_max = disk_info_style.is_removable_width_max;
        println!("{:kind_width_max$} {:name_width_max$} {:file_system_width_max$} {:mount_point_width_max$} {:>total_space_width_max$} {:>free_space_width_max$} {:>available_space_width_max$} {:is_removable_width_max$}",
                 "kind", "name", "file_system", "mount_point", "total_space", "free_space", "available_space", "is_removable");
        for disk in disk_info_vec {
            println!("{:kind_width_max$} {:name_width_max$} {:file_system_width_max$} {:mount_point_width_max$} {:>total_space_width_max$} {:>free_space_width_max$} {:>available_space_width_max$} {:is_removable_width_max$}",
                     disk.kind, disk.name, disk.file_system, disk.mount_point, disk.total_space, disk.free_space, disk.available_space, disk.is_removable);
        }

        println!()
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
