extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
const _: () = assert!(
    true,
    "std::is_trivially_copyable<decltype(std::ignore)>::value"
);
const _: () = assert!(
    true,
    "std::is_trivially_default_constructible<decltype(std::ignore)>::value"
);
pub unsafe fn test_nodiscard_0() -> () {
    return 8294_();
}
pub unsafe fn test_1() -> () {
    {
        let ignore_v: *const () = &(*std::cell::LazyCell::force_mut(&mut *&raw mut ignore_2));
    }
    {
        let res: *const () = &(*unimplemented!());
        const _: () = assert!(true, "noexcept(res = (std::ignore = 42))");;
        unimplemented!();
    }
    {
        let mut s: S_3 = S_3 { bf: 2_() };
        let res: *const () = &(*unimplemented!());
        unimplemented!();
    }
    {
        let mut copy: () = unimplemented!();
        let mut moved: () = unimplemented!();
    }
    {
        let mut copy: () = unimplemented!();
        unimplemented!();
        let mut moved: () = unimplemented!();
        unimplemented!();
    }
    {
        unimplemented!();
    }
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_3 {
    pub bf: (),
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
