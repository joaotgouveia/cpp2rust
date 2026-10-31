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
    pub count_: *mut i32,
}
impl TrackDtor {
    pub unsafe fn new(mut count: *mut i32) -> Self {
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
pub unsafe fn test_0() -> bool {
    let mut count: i32 = 0;
    {
        let mut tuple: () = unimplemented!();
        assert!(((count) == (0)));
    }
    assert!(((count) == (1)));
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
