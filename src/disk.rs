use std::mem;
use std::path::Path;

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

/// 必须要使用C语言风格的Path(以 \0 结尾的字符串)，否则macos不报错，但linux某些情况下会报错。
/// 注意: std::ffi::CString 不能用于 statvfs，会报错返回-1，用 std::ffi::OsStr 则可以。
pub fn call_statvfs(path: &String) -> Result<StatvfsResult, String> {
    // let path_ptr = path.as_ptr() as *const _;
    // let cpath_ptr = to_cpath(path);
    // let cpath_ptr = to_cpath(path).as_ptr() as *const _;
    // let cpath_ptr = "/Users/wyb\0".as_ptr() as *const c_char;
    // let cpath_ptr1 = path + String::from("\0");
    let mut cpath = path.clone();
    cpath.push('\0');
    let cpath_ptr = cpath.as_ptr()  as *const c_char;

    unsafe {
        let mut buf: statvfs = mem::zeroed();
        let result = statvfs(cpath_ptr, &mut buf); // 调用 statvfs 函数

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
            Err(format!("Error calling statvfs: code={} path={:?}", result, path))
        }
    }
}

/// 将路径转换为C风格字符串(末尾加\0)
pub(crate) fn to_cpath(path: &std::path::Path) -> Vec<u8> {
    use std::{ffi::OsStr, os::unix::ffi::OsStrExt};

    let path_os: &OsStr = path.as_ref();
    let mut cpath = path_os.as_bytes().to_vec();
    cpath.push(0);
    cpath
}


#[cfg(target_os = "macos")]
pub fn disk_info(path: &String) -> Result<StatvfsResult, String> {
    let mut result = call_statvfs(path)?;
    result.f_bsize = result.f_frsize;
    Ok(result)
}

#[cfg(not(target_os = "macos"))]
pub fn disk_info(path: &String) -> Result<StatvfsResult, String> {
    call_statvfs(path)
}

#[test]
fn test() {
    let path = Path::new("/Users/wyb");
    println!("path: {path:?}");

    let path = String::from("/Users/wyb");
    let result = call_statvfs(&path);
    match result {
        Ok(res) => { println!("test-Ok:{:?}", res) }
        Err(err) => { println!("test-Err: {}", err) }
    }

    let result = disk_info(&path);
    match result {
        Ok(res) => { println!("test-Ok:{:?}", res) }
        Err(err) => { println!("test-Err: {}", err) }
    }
}

#[test]
fn test_cpath() {
    let s = String::from("/Users/wyb");
    let a1 = Path::new(&s);
    println!("a1={a1:?}");
    let a2 = Path::new("/Users/wyb");
    println!("a2={a2:?}");
}
