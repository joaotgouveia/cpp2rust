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
    x_: (),
}
impl rand1_unsigned_int__0__10_ {
    pub unsafe fn min() -> () {
        return 0_();
    }
    pub unsafe fn max() -> () {
        return 10_();
    }
    pub unsafe fn new(mut sd: Option<()>) -> Self {
        let mut sd: () = sd.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { x_: sd };
        if ((this.x_) > (10_())) {
            this.x_ = 10_();
        }
        this
    }
    pub unsafe fn operator_call(&mut self) -> () {
        let mut r: () = self.x_;
        if ((self.x_) < (10_())) {
            self.x_.prefix_inc();
        } else {
            self.x_ = 0_();
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
    x_: (),
}
impl rand1_unsigned_int__0__100_ {
    pub unsafe fn min() -> () {
        return 0_();
    }
    pub unsafe fn max() -> () {
        return 100_();
    }
    pub unsafe fn new(mut sd: Option<()>) -> Self {
        let mut sd: () = sd.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self { x_: sd };
        if ((this.x_) > (100_())) {
            this.x_ = 100_();
        }
        this
    }
    pub unsafe fn operator_call(&mut self) -> () {
        let mut r: () = self.x_;
        if ((self.x_) < (100_())) {
            self.x_.prefix_inc();
        } else {
            self.x_ = 0_();
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
    x_: (),
}
impl rand1_unsigned_int__0__4294967295_ {
    pub unsafe fn min() -> () {
        return 0_();
    }
    pub unsafe fn max() -> () {
        return 4294967295_();
    }
    pub unsafe fn new(mut sd: ()) -> Self {
        let mut this = Self { x_: sd };
        if ((this.x_) > (4294967295_())) {
            this.x_ = 4294967295_();
        }
        this
    }
    pub unsafe fn operator_call(&mut self) -> () {
        let mut r: () = self.x_;
        if ((self.x_) < (4294967295_())) {
            self.x_.prefix_inc();
        } else {
            self.x_ = 0_();
        }
        return r;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct rand1_unsigned_int__2__3_ {
    x_: (),
}
impl rand1_unsigned_int__2__3_ {
    pub unsafe fn min() -> () {
        return 2_();
    }
    pub unsafe fn max() -> () {
        return 3_();
    }
    pub unsafe fn new(mut sd: ()) -> Self {
        let mut this = Self { x_: sd };
        if ((this.x_) > (3_())) {
            this.x_ = 3_();
        }
        this
    }
    pub unsafe fn operator_call(&mut self) -> () {
        let mut r: () = self.x_;
        if ((self.x_) < (3_())) {
            self.x_.prefix_inc();
        } else {
            self.x_ = 2_();
        }
        return r;
    }
}
pub unsafe fn test1_0() {
    let mut e: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test2_1() {
    let mut e: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test3_2() {
    let mut e: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test4_3() {
    let mut e: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test5_4() {
    let mut e: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test6_5() {
    let mut e: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test7_6() {
    let mut e: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test8_7() {
    let mut e: () = unimplemented!();
    unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test1_0() });
    (unsafe { test2_1() });
    (unsafe { test3_2() });
    (unsafe { test4_3() });
    (unsafe { test5_4() });
    (unsafe { test6_5() });
    (unsafe { test7_6() });
    (unsafe { test8_7() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
