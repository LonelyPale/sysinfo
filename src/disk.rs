use std::mem;
use std::path::Path;
use std::{ffi::OsStr, os::unix::ffi::OsStrExt};
use std::ffi::{CStr, CString};

extern crate libc;

use libc::{c_char, statvfs};

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
pub fn call_statvfs<S: AsRef<OsStr> + ?Sized>(path: &S) -> Result<StatvfsResult, String> {
    let cpath = to_cpath(path);
    let cpath_ptr = cpath.as_ptr() as *const c_char; //as_ptr()必须单独写一句，否则指针会因为中间对象被释放而失效。
    // let cpath_ptr = cpath.as_ptr() as *const _;
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
            let os_err = std::io::Error::last_os_error();
            let raw_os_error = os_err.raw_os_error().unwrap_or_default();
            let ret_err: String;
            if os_err.kind() == std::io::ErrorKind::Interrupted {
                let path = path.as_ref();
                ret_err = format!("Error calling statvfs: code={result} path={path:?}\nerr: std::io::ErrorKind::Interrupted raw_os_error={raw_os_error} os_err={os_err}");
            } else {
                let path = path.as_ref();
                ret_err = format!("Error calling statvfs: code={result} path={path:?}\nerr: raw_os_error={raw_os_error} os_err={os_err}");
            }
            Err(ret_err)
        }
    }
}

/// 将字符串路径转换为C风格字符串(末尾加\0)
/// 注意:
/// ```
///    //rust中以下代码是正确的，为什么？
///    let s = "abc";
///    let ss = s.to_string();
///    let ptr = ss.as_ptr();
///    println!(" ptr={ptr:?}");
///
///    //rust中以下代码是错误的，为什么？
///    let s1 = "abc";
///    let ptr1 = s.to_string().as_ptr();
///    println!("ptr1={ptr1:?}");
/// ```
/// 文心一言: 这里的问题在于as_ptr方法的调用位置。你试图在一个to_string()调用之后立即调用as_ptr()，但这是错误的。
/// 原因是to_string()方法返回一个新的String对象，这需要分配内存并复制原始字符串的字节。
/// 一旦to_string()方法完成并返回新的String对象，原始字符串就会被丢弃，因此你无法再获取其原始字节的指针。
///
/// as_ptr()返回指向向量缓冲区的原始指针，或者如果向量未分配，则返回对零大小读取有效的悬空原始指针。
/// 调用者必须确保向量比该函数返回的指针寿命更长，否则它将最终指向垃圾。修改向量可能会导致其缓冲区被重新分配，这也会使指向它的任何指针无效。
///
/// as_ptr()必须是单独的一句，不能用链式写法：s.to_string().as_ptr()。
/// 原因: to_string()会分配新对象，但新对象没有分配到变量，所以语句结束时新对象被释放了，而as_ptr()获取到的指针就指向了无效的数据。
///
/// 最佳实践: 只在要使用指针ptr的时候才在要使用的地方调用as_ptr()，不要提前调用，也不要在别的方法内调用as_ptr()再返回指针ptr给使用方。
/// 因为这样容易因所有权的问题而使返回的对象被提前释放，从而使as_ptr()得到的指针地址指向无效的内存。
pub(crate) fn to_cpath<S: AsRef<OsStr> + ?Sized>(s: &S) -> Vec<u8> {
    let path = Path::new(s);
    let path_os: &OsStr = path.as_ref();
    let mut cpath = path_os.as_bytes().to_vec();
    cpath.push(0);
    cpath
}

#[test]
fn test_to_cpath() {
    let s = "abc";
    let cpath = to_cpath(s);
    println!("cpath={cpath:?}");
    let cstr = unsafe { CStr::from_ptr(cpath.as_ptr() as *const _) };
    let cs = CString::from(cstr);
    println!("cs={:?}\n", cs.into_string());

    let s = String::from("123");
    let cpath = to_cpath(&s);
    println!("cpath={cpath:?}");
    let cstr = unsafe { CStr::from_ptr(cpath.as_ptr() as *const _) };
    let cs = CString::from(cstr);
    println!("cs={:?}\n", cs.into_string());

    let s = Path::new("xyz");
    let cpath = to_cpath(&s);
    println!("cpath={cpath:?}");
    let cstr = unsafe { CStr::from_ptr(cpath.as_ptr() as *const _) };
    let cs = CString::from(cstr);
    println!("cs={:?}\n", cs.into_string());
}

#[test]
fn test() {
    let path = String::from("/Users/wyb");
    let result = call_statvfs(&path);
    match result {
        Ok(res) => { println!("test-Ok: {:?}", res) }
        Err(err) => { println!("test-Err: {}", err) }
    }

    let result = disk_info(&path);
    match result {
        Ok(res) => { println!("test-Ok:{:?}", res) }
        Err(err) => { println!("test-Err: {}", err) }
    }
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
fn test_ptr() {
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
