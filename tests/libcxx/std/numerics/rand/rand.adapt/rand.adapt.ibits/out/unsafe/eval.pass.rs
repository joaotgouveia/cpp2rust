extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct rand1_unsigned_int__0__10_ {
    x_: u32,
}
impl rand1_unsigned_int__0__10_ {
    pub unsafe fn min() -> u32 {
        return 0_u32;
    }
    pub unsafe fn max() -> u32 {
        return 10_u32;
    }
    pub unsafe fn new(mut sd: Option<u32>) -> Self {
        let mut sd: u32 = sd.unwrap_or_else(|| unsafe { 0_u32 });
        let mut this = Self { x_: sd };
        if ((this.x_) > (10_u32)) {
            this.x_ = 10_u32;
        }
        this
    }
    pub unsafe fn operator_call(&mut self) -> u32 {
        let mut r: u32 = self.x_;
        if ((self.x_) < (10_u32)) {
            self.x_.prefix_inc();
        } else {
            self.x_ = 0_u32;
        }
        return r;
    }
}
impl Default for rand1_unsigned_int__0__10_ {
    fn default() -> Self {
        unsafe { rand1_unsigned_int__0__10_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct rand1_unsigned_int__0__100_ {
    x_: u32,
}
impl rand1_unsigned_int__0__100_ {
    pub unsafe fn min() -> u32 {
        return 0_u32;
    }
    pub unsafe fn max() -> u32 {
        return 100_u32;
    }
    pub unsafe fn new(mut sd: Option<u32>) -> Self {
        let mut sd: u32 = sd.unwrap_or_else(|| unsafe { 0_u32 });
        let mut this = Self { x_: sd };
        if ((this.x_) > (100_u32)) {
            this.x_ = 100_u32;
        }
        this
    }
    pub unsafe fn operator_call(&mut self) -> u32 {
        let mut r: u32 = self.x_;
        if ((self.x_) < (100_u32)) {
            self.x_.prefix_inc();
        } else {
            self.x_ = 0_u32;
        }
        return r;
    }
}
impl Default for rand1_unsigned_int__0__100_ {
    fn default() -> Self {
        unsafe { rand1_unsigned_int__0__100_::new(None) }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct rand1_unsigned_int__0__4294967295_ {
    x_: u32,
}
impl rand1_unsigned_int__0__4294967295_ {
    pub unsafe fn min() -> u32 {
        return 0_u32;
    }
    pub unsafe fn max() -> u32 {
        return 4294967295_u32;
    }
    pub unsafe fn new(mut sd: u32) -> Self {
        let mut this = Self { x_: sd };
        if ((this.x_) > (4294967295_u32)) {
            this.x_ = 4294967295_u32;
        }
        this
    }
    pub unsafe fn operator_call(&mut self) -> u32 {
        let mut r: u32 = self.x_;
        if ((self.x_) < (4294967295_u32)) {
            self.x_.prefix_inc();
        } else {
            self.x_ = 0_u32;
        }
        return r;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct rand1_unsigned_int__2__3_ {
    x_: u32,
}
impl rand1_unsigned_int__2__3_ {
    pub unsafe fn min() -> u32 {
        return 2_u32;
    }
    pub unsafe fn max() -> u32 {
        return 3_u32;
    }
    pub unsafe fn new(mut sd: u32) -> Self {
        let mut this = Self { x_: sd };
        if ((this.x_) > (3_u32)) {
            this.x_ = 3_u32;
        }
        this
    }
    pub unsafe fn operator_call(&mut self) -> u32 {
        let mut r: u32 = self.x_;
        if ((self.x_) < (3_u32)) {
            self.x_.prefix_inc();
        } else {
            self.x_ = 2_u32;
        }
        return r;
    }
}
pub unsafe fn test1_0() {
    let mut e: () = unimplemented!();
    assert!(((unimplemented!()) == (6958_())));
}
pub unsafe fn test2_1() {
    let mut e: () = unimplemented!();
    assert!(((unimplemented!()) == (66_())));
}
pub unsafe fn test3_2() {
    let mut e: () = unimplemented!();
    assert!(((unimplemented!()) == (5_())));
}
pub unsafe fn test4_3() {
    let mut e: () = unimplemented!();
    assert!(((unimplemented!()) == (1_())));
}
pub unsafe fn test5_4() {
    let mut e: () = unimplemented!();
    assert!(((unimplemented!()) == (1_())));
}
pub unsafe fn test6_5() {
    let mut e: () = unimplemented!();
    assert!(((unimplemented!()) == (1365_())));
}
pub unsafe fn test7_6() {
    let mut e: () = unimplemented!();
    assert!(((unimplemented!()) == (2863311530_u32)));
}
pub unsafe fn test8_7() {
    let mut e: () = unimplemented!();
    assert!(((unimplemented!()) == (16470362623952407241_u64)));
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test1_0() });
    (unsafe { test2_1() });
    (unsafe { test3_2() });
    (unsafe { test4_3() });
    (unsafe { test5_4() });
    (unsafe { test6_5() });
    (unsafe { test7_6() });
    (unsafe { test8_7() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
