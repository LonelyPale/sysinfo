use colored::Colorize;
use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind, Components, Disks};
use crate::common::PrettySize;

#[derive(Debug)]
pub struct SysInfo {
    system: System,
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
        let disks = Disks::new_with_refreshed_list();
        for disk in &disks {
            println!("{disk:?}");

            let kind = disk.kind();
            let name = disk.name();
            let file_system = disk.file_system();
            let mount_point = disk.mount_point();
            let total_space = disk.total_space();
            let available_space = disk.available_space();
            let is_removable = disk.is_removable();
            println!("{} {} {} {} {} {} {}", kind, name.to_str().unwrap_or_default(), file_system.to_str().unwrap_or_default(), mount_point.to_str().unwrap_or_default(), total_space.pretty_size(), available_space.pretty_size(), is_removable);
            println!();
        }

        println!()
    }
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
