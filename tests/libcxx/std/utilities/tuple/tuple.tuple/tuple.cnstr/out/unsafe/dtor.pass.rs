extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, DestructorUnsafe, Default)]
pub struct TrackDtor {
    pub count_: *mut (),
}
impl TrackDtor {
    pub unsafe fn new(mut count: *mut ()) -> Self {
        let mut this = Self { count_: count };
        this
    }
    pub unsafe fn move_from(that: *mut TrackDtor) -> Self {
        let mut this = Self {
            count_: (*that).count_,
        };
        (*that).count_ = std::ptr::null_mut();
        this
    }
    pub unsafe fn destructor(&mut self) {
        if !(self.count_).is_null() {
            (*self.count_).prefix_inc();
        }
    }
}
const _: () = assert!(true, "!std::is_trivially_destructible<TrackDtor>::value");
const _: () = assert!(true, "std::is_trivially_destructible<std::tuple<>>::value");
const _: () = assert!(
    true,
    "std::is_trivially_destructible<std::tuple<void*>>::value"
);
const _: () = assert!(
    true,
    "std::is_trivially_destructible<std::tuple<int, float>>::value"
);
const _: () = assert!(
    true,
    "!std::is_trivially_destructible<std::tuple<std::string>>::value"
);
const _: () = assert!(
    true,
    "!std::is_trivially_destructible<std::tuple<int, std::string>>::value"
);
pub unsafe fn test_0() -> () {
    let mut count: () = 0_();
    {
        let mut tuple: () = unimplemented!();
        unimplemented!();
    }
    unimplemented!();
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
