extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub fn foo_0() -> bool {
    return true;
}
pub fn bar_1() -> bool {
    return true;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let mut i1: i32 = 0;
    let mut i2: i32 = 1;
    {
        let __s = {
            let __s = libcc2rs::cout();
            let _ = write!(__s, "{}", true as i32);
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    {
        let __s = {
            let __s = libcc2rs::cout();
            let _ = write!(__s, "{}", false as i32);
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    {
        let __s = {
            let __s = libcc2rs::cout();
            let _ = write!(__s, "{}", (i1 != i2) as i32);
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    {
        let __s = {
            let __s = libcc2rs::cout();
            let _ = write!(__s, "{}", (i1 == i2) as i32);
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    {
        let __s = {
            let __s = libcc2rs::cout();
            let _ = write!(__s, "{}", ({ foo_0() }) as i32);
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    {
        let __s = {
            let __s = libcc2rs::cout();
            let _ = write!(__s, "{}", ({ bar_1() }) as i32);
            __s
        };
        let _ = __s.write_all(&[('\n' as i8) as u8]);
        __s
    };
    return 0;
}
pub fn __cpp2rust_init_globals() {}
