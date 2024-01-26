use std::ffi::CString;
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

    // let mut cpath = path.clone();
    // cpath.push('\0');
    // let cpath_ptr = cpath.as_ptr() as *const c_char;

    let path = Path::new(&path);
    let cpath_ptr = to_cpath(path);
    let cpath_ptr = cpath_ptr.as_ptr();

    // let path = CString::new(path);
    // let cpath_ptr = path.unwrap();
    unsafe {
        let cpath_ptr = cpath_ptr;
        let value = *cpath_ptr;
        let char_value: char = value as char;
        println!("第一个元素是: {value} = {char_value}");

        let format = CString::new("printf(): %s\n").unwrap();
        let result = libc::printf(format.as_ptr(), cpath_ptr);
        println!("Printed {} bytes", result);

        let mut buf: statvfs = mem::zeroed();
        let result = statvfs(cpath_ptr as *const c_char, &mut buf); // 调用 statvfs 函数

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
            let os_err = std::io::Error::last_os_error();
            let raw_os_error = os_err.raw_os_error().unwrap_or_default();
            let ret_err: String;
            if os_err.kind() == std::io::ErrorKind::Interrupted {
                ret_err = format!("Error calling statvfs: code={result} path={result:?}\nstd::io::ErrorKind::Interrupted raw_os_error={raw_os_error} os_err={os_err}");
            } else {
                ret_err = format!("Error calling statvfs: code={result} path={result:?}\nraw_os_error={raw_os_error} os_err={os_err}");
            }
            Err(ret_err)
        }
    }
}

/// 将路径转换为C风格字符串(末尾加\0)
fn to_cpath(path: &Path) -> Vec<u8> {
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
    // let path = Path::new("/Users/wyb");
    // println!("path: {path:?}");

    let path = String::from("/Users/wyb");
    let result = call_statvfs(&path);
    match result {
        Ok(res) => { println!("test-Ok: {:?}", res) }
        Err(err) => { println!("test-Err: {:?}", err) }
    }

    // let result = disk_info(&path);
    // match result {
    //     Ok(res) => { println!("test-Ok:{:?}", res) }
    //     Err(err) => { println!("test-Err: {}", err) }
    // }
}

#[test]
fn test_path() {
    let s = String::from("/Users/wyb");
    let a1 = Path::new(&s);
    println!("a1={a1:?}");
    let a2 = Path::new("/Users/wyb");
    println!("a2={a2:?}");
}

#[test]
fn test_ptr_slice() {
    let slice = [111, 222, 333, 444, 555];
    let ptr = slice.as_ptr();
    unsafe {
        // 在这里使用ptr，例如传递给C函数或解引用以访问元素
        let value = *ptr;
        println!("第一个元素是: {value}");
    }
}

#[test]
fn test_ptr_str() {
    let str = "abcdef\0";
    let ptr = str.as_ptr();
    unsafe {
        let value = *ptr;
        let char_value: char = value as char;
        println!("第1个元素是: {value} = {char_value}");

        let next_ptr = ptr.add(1);// 指针加法，移动到下一个元素
        let value = *next_ptr;
        let char_value: char = value as char;
        println!("下1个元素是: {value} = {char_value}");

        let last_ptr = next_ptr.sub(1);
        let value = *last_ptr;
        let char_value: char = value as char;
        println!("上1个元素是: {value} = {char_value}");

        let new_ptr = ptr.offset(6);
        let value = *new_ptr;
        let char_value: char = value as char;
        println!("第x个元素是: {value} = {char_value}");
    }
}

#[test]
fn test_libc_printf() {
    // 使用C风格的字符串格式化
    let format = "printf() => 整数：%d，字符串：%s\r\n\0" as *const str as *const c_char;
    let number = 88;
    let string = "Hello, world\0" as *const str as *const c_char;

    println!("===== start =====");
    // 调用printf，异步
    unsafe { libc::printf(format, number, string); }
    println!("=====  end  =====");
}

#[test]
fn test_libc_printf_cs() {
    let format = CString::new("Hello, %s! num=%d \n").unwrap();
    let result = unsafe { libc::printf(format.as_ptr(), "Rust", 88) };
    println!("Printed {} bytes", result);
}

#[test]
fn test_ptr_ok() {
    let x = vec![1, 2, 4];
    let x_ptr = x.as_ptr();

    unsafe {
        for i in 0..x.len() {
            assert_eq!(*x_ptr.add(i), 1 << i);
            println!("i={i} {} {}", *x_ptr.add(i), 1 << i);
        }
    }
}

#[test]
fn test_ptr_err() {
    let x = vec![1, 2, 4];
    let x_ptr = x.as_ptr();
    let x_ptr = vec![1, 2, 4].as_ptr();

    unsafe {
        for i in 0..3 {
            println!("i={i} {} {}", *x_ptr.add(i), 1 << i);
            // assert_eq!(*x_ptr.add(i), 1 << i);
        }
    }
}

#[test]
fn test_ptr_ok2() {
    let s = "abc".to_string();
    let s_ptr = s.as_ptr();

    unsafe {
        for i in 0..s.len() {
            let v = *s_ptr.add(i);
            let c = v as char;
            println!("i={i} {} {}", v, c);
        }
    }
}

#[test]
fn test_ptr_err2() {
    //rust中以下代码是正确的，为什么？
    let s = "abc".to_string();
    let s_ptr = s.as_ptr();

    //rust中以下代码是错误的，为什么？
    let s_ptr = "abc".to_string().as_ptr();


    unsafe {
        for i in 0..s.len() {
            let v = *s_ptr.add(i);
            let c = v as char;
            println!("i={i} {} {}", v, c);
        }
    }
}

#[test]
fn test_(){
    //rust中以下代码是正确的，为什么？
    let s = "abc";
    let ss = s.to_string();
    let ptr = ss.as_ptr();
    println!(" ptr={ptr:?}");

    //rust中以下代码是错误的，为什么？
    let s1 = "abc";
    let ptr1 = s.to_string().as_ptr();
    println!("ptr1={ptr1:?}");

    unsafe {
        for i in 0..s.len() {
            let v = *ptr.add(i);
            let c = v as char;
            println!("i={i} {} {}", v, c);
        }
    }
}
