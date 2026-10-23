extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct __gnu_test_regex_expected_fail {}
pub static mut regex_error_internal_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { ((-1_()) as ()) });
pub static mut regex_error_codes_1: std::cell::LazyCell<[*const (); 13]> =
    std::cell::LazyCell::new(|| unsafe {
        [
            c"error_collate".as_ptr(),
            c"error_ctype".as_ptr(),
            c"error_escape".as_ptr(),
            c"error_backref".as_ptr(),
            c"error_brack".as_ptr(),
            c"error_paren".as_ptr(),
            c"error_brace".as_ptr(),
            c"error_badbrace".as_ptr(),
            c"error_range".as_ptr(),
            c"error_space".as_ptr(),
            c"error_badrepeat".as_ptr(),
            c"error_complexity".as_ptr(),
            c"error_stack".as_ptr(),
        ]
    });
pub unsafe fn show_regex_error_codes_2() {
    let tab: () = ('\t' as ());
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
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn regex_sanity_check_3(__res: *const (), mut __f: Option<()>, mut __error: Option<()>) {
    let mut __f: () =
        __f.unwrap_or_else(|| unsafe { (*std::cell::LazyCell::force_mut(&mut *&raw mut basic_4)) });
    let mut __error: () = __error.unwrap_or_else(|| unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut regex_error_internal_0))
    });
    let mut reo: () = unimplemented!();
    let mut n: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test01_5() {
    let mut sre0: () = unimplemented!();
    let mut sre1: () = unimplemented!();
    let mut sre2: () = unimplemented!();
    let mut re0: () = unimplemented!();
    let mut re1: () = unimplemented!();
    let mut re2: () = unimplemented!();
    (unsafe {
        regex_sanity_check_3(
            &sre0,
            Some((*std::cell::LazyCell::force_mut(&mut *&raw mut ECMAScript_6))),
            None,
        )
    });
    (unsafe { regex_sanity_check_3(&sre1, None, None) });
    (unsafe { regex_sanity_check_3(&sre2, None, None) });
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> () {
    (unsafe { test01_5() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const regex_error_internal_0);
    std::cell::LazyCell::force(&*&raw const regex_error_codes_1);
}
