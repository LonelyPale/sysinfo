use std::ffi::CString;
use std::mem;

extern crate libc;

use libc::{c_char, c_int, size_t, statvfs};
use libc::{c_uint, c_void, uint32_t, attrlist, getattrlist as other_getattrlist, stat};

use crate::common::PrettySize;

pub fn fill_statvfs(fp: String) {
    let path = fp + "\0";
    let path = path.as_ptr() as *const c_char;

    unsafe {
        let mut buf: statvfs = mem::zeroed();

        // 调用 statvfs 函数
        let result = statvfs(path, &mut buf as *mut statvfs);

        if result == 0 {
            let size = 4096;
            // let size = u64::from(buf.f_bsize);
            let blocks = u64::from(buf.f_blocks);
            let free = u64::from(buf.f_bfree);
            let avail = u64::from(buf.f_bavail);

            // 打印获取的信息
            println!("Block size: {} {} {}", size.pretty_size(), size, buf.f_bsize);
            println!("Total blocks: {} {} {}", (blocks * size).pretty_size(), blocks, buf.f_blocks);
            println!("Free blocks: {} {} {}", (free * size).pretty_size(), free, buf.f_bfree);
            println!("Avail blocks: {} {} {}", (avail * size).pretty_size(), avail, buf.f_bavail);
            // 其他信息...
        } else {
            // 错误处理
            println!("Error calling statvfs");
        }
    }
}


#[test]
fn test2() {
    let path = "/path/to/your/file\0".as_ptr() as *const c_char;

    let attr_list: attrlist = attrlist {
        bitmapcount: 5, // 设置你需要获取的属性个数
        reserved: 0,
        commonattr: 0xFFFFFFFF, // 通用属性，例如文件大小、创建时间等
        volattr: 0,
        dirattr: 0,//??
        fileattr: 0,
        forkattr: 0,
    };

    unsafe {
        let mut attr_buf: stat = mem::zeroed();

        // 调用 getattrlist 函数
        // let result = getattrlist(
        //     path,
        //     &attr_list as *const attrlist,
        //     &mut attr_buf as *mut stat as *mut c_void,
        //     size_of::<stat>() as size_t,
        //     c_uint::MAX,
        // );

        let result = 2;
        if result == 0 {
            // 获取到属性后，可以通过 stat 结构体中的字段来访问
            println!("File Size: {}", attr_buf.st_size);
            // 其他属性...
        } else {
            // 错误处理
            println!("Error calling getattrlist");
        }
    }
}

extern "C" {
    fn getattrlist(file: *const c_char, list: *mut c_char, buffer: *mut c_char, buffer_size: size_t, options: c_int, file_info: *mut c_int) -> c_int;
}

#[test]
fn test3() {
    let file = CString::new("/path/to/file").unwrap();
    let mut list = [0u8; 1024]; // Buffer to store attributes
    let mut file_info = 0; // File information
    let result = unsafe {
        getattrlist(file.as_ptr(),
                    list.as_mut_ptr() as *mut c_char,
                    list.as_mut_ptr() as *mut c_char,
                    list.len() as size_t,
                    0,
                    &mut file_info)
    };
    if result == 0 {
        println!("{:?}", list);
        println!("{:?}", file_info);
    } else {
        eprintln!("err {}", result)
    }
}


#[test]
fn test() {
    fill_statvfs("/Users/wyb".to_string());
}
