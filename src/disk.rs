use std::mem;

extern crate libc;

use libc::{c_char, c_int, size_t, statvfs};
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
fn test() {
    fill_statvfs("/Users/wyb".to_string());
}
