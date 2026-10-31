extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(*const libc::c_char , to_pconstlibcc_char)]
#[cpp_into(*const i32 , to_pconsti32)]
#[cpp_into(*const u8 , to_pconstu8)]
#[cpp_into(*const u16 , to_pconstu16)]
#[cpp_into(*const u32 , to_pconstu32)]
pub struct MultiStringType {
    pub w_: *const i32,
    pub wn_: usize,
    pub u8_: *const u8,
    pub u8n_: usize,
    pub u16_: *const u16,
    pub u16n_: usize,
    pub u32_: *const u32,
    pub u32n_: usize,
    pub s_: *const libc::c_char,
    pub sn_: usize,
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const i32,
        mut wn: usize,
        mut u8: *const u8,
        mut u8n: usize,
        mut u16: *const u16,
        mut u16n: usize,
        mut u32: *const u32,
        mut u32n: usize,
        mut s: *const libc::c_char,
        mut sn: usize,
    ) -> Self {
        let mut this = Self {
            w_: w,
            wn_: wn,
            u8_: u8,
            u8n_: u8n,
            u16_: u16,
            u16n_: u16n,
            u32_: u32,
            u32n_: u32n,
            s_: s,
            sn_: sn,
        };
        this
    }
    pub unsafe fn as_ptr_2(&self, mut _a0: *const libc::c_char) -> *const libc::c_char {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const i32) -> *const i32 {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const u8) -> *const u8 {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const u16) -> *const u16 {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const u32) -> *const u32 {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const libc::c_char) -> usize {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const i32) -> usize {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const u8) -> usize {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const u16) -> usize {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const u32) -> usize {
        return self.u32n_;
    }
    pub unsafe fn to_pconstlibcc_char(&self) -> *const libc::c_char {
        return self.s_;
    }
    pub unsafe fn to_pconsti32(&self) -> *const i32 {
        return self.w_;
    }
    pub unsafe fn to_pconstu8(&self) -> *const u8 {
        return self.u8_;
    }
    pub unsafe fn to_pconstu16(&self) -> *const u16 {
        return self.u16_;
    }
    pub unsafe fn to_pconstu32(&self) -> *const u32 {
        return self.u32_;
    }
}
pub unsafe fn test_type_0() {
    let mut ii1: () = unimplemented!();
    let mut ii2: () = unimplemented!();
    let mut ii4: () = ii1;
    let mut cii: () = unimplemented!();
    assert!(((ii1) == (ii2)));
    assert!(((ii1) == (ii4)));
    assert!(((ii1) == (cii)));
    assert!(!((ii1) != (ii2)));
    assert!(!((ii1) != (cii)));
    let mut c: () = unimplemented!();
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) != (unimplemented!())));
    assert!(unimplemented!() != unimplemented!());
    assert!(((unimplemented!()) != (unimplemented!())));
    assert!(unimplemented!() != unimplemented!());
    let mut r1: std::cmp::Ordering = std::cmp::Ord::cmp(&(ii1), &(ii2));
    assert!(r1 == std::cmp::Ordering::Equal);
    let mut r2: std::cmp::Ordering = std::cmp::Ord::cmp(&(ii1), &(ii2));
    assert!(r2 == std::cmp::Ordering::Equal);
}
pub unsafe fn test_type_1() {
    let mut ii1: () = unimplemented!();
    let mut ii2: () = unimplemented!();
    let mut ii4: () = ii1;
    let mut cii: () = unimplemented!();
    assert!(((ii1) == (ii2)));
    assert!(((ii1) == (ii4)));
    assert!(((ii1) == (cii)));
    assert!(!((ii1) != (ii2)));
    assert!(!((ii1) != (cii)));
    let mut c: () = unimplemented!();
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) != (unimplemented!())));
    assert!(unimplemented!() != unimplemented!());
    assert!(((unimplemented!()) != (unimplemented!())));
    assert!(unimplemented!() != unimplemented!());
    let mut r1: std::cmp::Ordering = std::cmp::Ord::cmp(&(ii1), &(ii2));
    assert!(r1 == std::cmp::Ordering::Equal);
    let mut r2: std::cmp::Ordering = std::cmp::Ord::cmp(&(ii1), &(ii2));
    assert!(r2 == std::cmp::Ordering::Equal);
}
pub unsafe fn test_type_2() {
    let mut ii1: () = unimplemented!();
    let mut ii2: () = unimplemented!();
    let mut ii4: () = ii1;
    let mut cii: () = unimplemented!();
    assert!(((ii1) == (ii2)));
    assert!(((ii1) == (ii4)));
    assert!(((ii1) == (cii)));
    assert!(!((ii1) != (ii2)));
    assert!(!((ii1) != (cii)));
    let mut c: () = unimplemented!();
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) != (unimplemented!())));
    assert!(unimplemented!() != unimplemented!());
    assert!(((unimplemented!()) != (unimplemented!())));
    assert!(unimplemented!() != unimplemented!());
    let mut r1: std::cmp::Ordering = std::cmp::Ord::cmp(&(ii1), &(ii2));
    assert!(r1 == std::cmp::Ordering::Equal);
    let mut r2: std::cmp::Ordering = std::cmp::Ord::cmp(&(ii1), &(ii2));
    assert!(r2 == std::cmp::Ordering::Equal);
}
pub unsafe fn test_type_3() {
    let mut ii1: () = unimplemented!();
    let mut ii2: () = unimplemented!();
    let mut ii4: () = ii1;
    let mut cii: () = unimplemented!();
    assert!(((ii1) == (ii2)));
    assert!(((ii1) == (ii4)));
    assert!(((ii1) == (cii)));
    assert!(!((ii1) != (ii2)));
    assert!(!((ii1) != (cii)));
    let mut c: () = unimplemented!();
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) != (unimplemented!())));
    assert!(unimplemented!() != unimplemented!());
    assert!(((unimplemented!()) != (unimplemented!())));
    assert!(unimplemented!() != unimplemented!());
    let mut r1: std::cmp::Ordering = std::cmp::Ord::cmp(&(ii1), &(ii2));
    assert!(r1 == std::cmp::Ordering::Equal);
    let mut r2: std::cmp::Ordering = std::cmp::Ord::cmp(&(ii1), &(ii2));
    assert!(r2 == std::cmp::Ordering::Equal);
}
pub unsafe fn test_type_4() {
    let mut ii1: () = unimplemented!();
    let mut ii2: () = unimplemented!();
    let mut ii4: () = ii1;
    let mut cii: () = unimplemented!();
    assert!(((ii1) == (ii2)));
    assert!(((ii1) == (ii4)));
    assert!(((ii1) == (cii)));
    assert!(!((ii1) != (ii2)));
    assert!(!((ii1) != (cii)));
    let mut c: () = unimplemented!();
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (unimplemented!())));
    assert!(unimplemented!());
    assert!(((unimplemented!()) != (unimplemented!())));
    assert!(unimplemented!() != unimplemented!());
    assert!(((unimplemented!()) != (unimplemented!())));
    assert!(unimplemented!() != unimplemented!());
    let mut r1: std::cmp::Ordering = std::cmp::Ord::cmp(&(ii1), &(ii2));
    assert!(r1 == std::cmp::Ordering::Equal);
    let mut r2: std::cmp::Ordering = std::cmp::Ord::cmp(&(ii1), &(ii2));
    assert!(r2 == std::cmp::Ordering::Equal);
}
pub unsafe fn test_5() -> bool {
    (unsafe { test_type_0() });
    (unsafe { test_type_1() });
    (unsafe { test_type_2() });
    (unsafe { test_type_3() });
    (unsafe { test_type_4() });
    return true;
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
    (unsafe { test_5() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
