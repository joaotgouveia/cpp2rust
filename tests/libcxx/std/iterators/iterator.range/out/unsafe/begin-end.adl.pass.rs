extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn begin_0(_a0: *mut [ArrayHijacker; 3]) -> () {
    return 42_();
}
pub unsafe fn end_1(_a0: *mut [ArrayHijacker; 3]) -> () {
    return 42_();
}
pub unsafe fn begin_2(_a0: *const [ArrayHijacker; 3]) -> () {
    return 42_();
}
pub unsafe fn end_3(_a0: *const [ArrayHijacker; 3]) -> () {
    return 42_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ArrayHijacker {}
pub unsafe fn begin_4(_a0: *mut ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn end_5(_a0: *mut ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn begin_6(_a0: *const ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn end_7(_a0: *const ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn cbegin_8(_a0: *mut ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn cend_9(_a0: *mut ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn cbegin_10(_a0: *const ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn cend_11(_a0: *const ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn rbegin_12(_a0: *mut ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn rend_13(_a0: *mut ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn rbegin_14(_a0: *const ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn rend_15(_a0: *const ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn crbegin_16(_a0: *mut ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn crend_17(_a0: *mut ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn crbegin_18(_a0: *const ContainerHijacker) -> () {
    return 42_();
}
pub unsafe fn crend_19(_a0: *const ContainerHijacker) -> () {
    return 42_();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ContainerHijacker {
    pub a_: *mut (),
}
impl ContainerHijacker {
    pub unsafe fn begin(&self) -> *mut () {
        return self.a_;
    }
    pub unsafe fn end(&self) -> *mut () {
        return self.a_.offset((3_()) as isize);
    }
    pub unsafe fn rbegin(&self) -> *mut () {
        return self.a_;
    }
    pub unsafe fn rend(&self) -> *mut () {
        return self.a_.offset((3_()) as isize);
    }
}
pub unsafe fn test_20() -> () {
    {
        let mut a: [ArrayHijacker; 3] = [
            <ArrayHijacker>::default(),
            <ArrayHijacker>::default(),
            <ArrayHijacker>::default(),
        ];
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a: [(); 3] = [unimplemented!(), unimplemented!(), unimplemented!()];
        let mut c: ContainerHijacker = ContainerHijacker { a_: a.as_mut_ptr() };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_20() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
