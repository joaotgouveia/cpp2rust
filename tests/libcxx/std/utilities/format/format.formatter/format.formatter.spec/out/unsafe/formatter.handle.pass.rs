extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_format_context_create_0(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_1(out_it, args.clone(), None) });
}
pub type color = ();
pub const color_black: color = 0;
pub const color_red: color = 1;
pub const color_gold: color = 2;
pub static mut color_names_2: std::cell::LazyCell<[*const (); 3]> =
    std::cell::LazyCell::new(|| unsafe { [c"black".as_ptr(), c"red".as_ptr(), c"gold".as_ptr()] });
pub unsafe fn test_3(mut expected: (), mut fmt: (), mut arg: color, mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: std_formatter_color__char_ = <std_formatter_color__char_>::default();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = (unsafe { (formatter as std___formatter_string_char_).parse(&mut parse_ctx) });
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () =
        (unsafe { test_format_context_create_0(out.clone(), unimplemented!()) });
    (unsafe {
        std_formatter_color__char_ :: format_1_class_std_basic_format_context_class_std_back_insert_iterator_std_string___char_ ( & formatter  , arg , & mut format_ctx , )
    });
    unimplemented!();
}
pub unsafe fn test_termination_condition_4(mut expected: (), mut f: (), mut arg: color) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_3(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_3(unimplemented!(), unimplemented!(), arg, 0_()) });
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
    (unsafe { test_termination_condition_4(unimplemented!(), unimplemented!(), color_black) });
    (unsafe { test_termination_condition_4(unimplemented!(), unimplemented!(), color_red) });
    (unsafe { test_termination_condition_4(unimplemented!(), unimplemented!(), color_gold) });
    return 0_();
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std_formatter_color__char_;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const color_names_2);
}
