use std::mem;

extern crate libc;

use libc::{c_char, statvfs};

#[derive(Debug)]
pub struct StatvfsResult {
    pub f_bsize: u64,
    pub f_frsize: u64,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_favail: u64,
    pub f_fsid: u64,
    pub f_flag: u64,
    pub f_namemax: u64,
}

pub fn call_statvfs(path: String) -> Result<StatvfsResult, String> {
    let path = path.as_ptr() as *const c_char;

    unsafe {
        let mut buf: statvfs = mem::zeroed();
        let result = statvfs(path, &mut buf as *mut statvfs); // 调用 statvfs 函数

        if result == 0 {//调用成功
            let res = StatvfsResult {
                f_bsize: u64::from(buf.f_bsize),
                f_frsize: u64::from(buf.f_frsize),
                f_blocks: u64::from(buf.f_blocks),
                f_bfree: u64::from(buf.f_bfree),
                f_bavail: u64::from(buf.f_bavail),
                f_files: u64::from(buf.f_files),
                f_ffree: u64::from(buf.f_ffree),
                f_favail: u64::from(buf.f_favail),
                f_fsid: u64::from(buf.f_fsid),
                f_flag: u64::from(buf.f_flag),
                f_namemax: u64::from(buf.f_namemax),
            };
            Ok(res)
        } else {//错误处理
            Err(format!("Error calling statvfs: {}", result))
        }
    }
}

#[cfg(target_os = "macos")]
pub fn disk_info(path: String) -> Result<StatvfsResult, String> {
    let mut result = call_statvfs(path)?;
    result.f_bsize = result.f_frsize;
    Ok(result)
}

#[cfg(not(target_os = "macos"))]
pub fn disk_info(path: String) -> Result<StatvfsResult, String> {
    call_statvfs(path)
}

#[test]
fn test() {
    let path = "/Users/wyb".to_string();
    let result = call_statvfs(path.clone());
    match result {
        Ok(res) => { println!("test-Ok:{:?}", res) }
        Err(err) => { println!("test-Err: {}", err) }
    }

    let result = disk_info(path.clone());
    match result {
        Ok(res) => { println!("test-Ok:{:?}", res) }
        Err(err) => { println!("test-Err: {}", err) }
    }
}
