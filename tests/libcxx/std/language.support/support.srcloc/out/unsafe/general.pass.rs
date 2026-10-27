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
    "std::is_nothrow_move_constructible_v<std::source_location>"
);
const _: () = assert!(
    true,
    "std::is_nothrow_move_assignable_v<std::source_location>"
);
const _: () = assert!(true, "std::is_nothrow_swappable_v<std::source_location>");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
pub static mut empty_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
const _: () = assert!(true, "empty.line() == 0");
const _: () = assert!(true, "empty.column() == 0");
const _: () = assert!(true, "empty.file_name()[0] == \'\\0\'");
const _: () = assert!(true, "empty.function_name()[0] == \'\\0\'");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
const _: () = assert!(true, "");
pub static mut line_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut column_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut file_3: std::cell::LazyCell<*const ()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut function_4: std::cell::LazyCell<*const ()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut cur_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
const _: () = assert!(true, "cur.line() == 1000");
const _: () = assert!(true, "cur.column() > 0");
const _: () = assert!(
    true,
    "cur.file_name()[0] == \'s\' && cur.file_name()[1] == \'s\' && cur.file_name()[2] == \'\\0\'"
);
const _: () = assert!(true, "cur.function_name()[0] == \'\\0\'");
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
    let mut local: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut local2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const empty_0);
    std::cell::LazyCell::force(&*&raw const line_1);
    std::cell::LazyCell::force(&*&raw const column_2);
    std::cell::LazyCell::force(&*&raw const file_3);
    std::cell::LazyCell::force(&*&raw const function_4);
    std::cell::LazyCell::force(&*&raw const cur_5);
}
