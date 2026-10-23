extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn where_0(_a0: *const ()) {}
pub unsafe fn test_1() {
    const _: () = assert!(true, "(BR::icase == std::regex_constants::icase)");;
    const _: () = assert!(true, "(BR::nosubs == std::regex_constants::nosubs)");;
    const _: () = assert!(true, "(BR::optimize == std::regex_constants::optimize)");;
    const _: () = assert!(true, "(BR::collate == std::regex_constants::collate)");;
    const _: () = assert!(true, "(BR::ECMAScript == std::regex_constants::ECMAScript)");;
    const _: () = assert!(true, "(BR::basic == std::regex_constants::basic)");;
    const _: () = assert!(true, "(BR::extended == std::regex_constants::extended)");;
    const _: () = assert!(true, "(BR::awk == std::regex_constants::awk)");;
    const _: () = assert!(true, "(BR::grep == std::regex_constants::grep)");;
    const _: () = assert!(true, "(BR::egrep == std::regex_constants::egrep)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut icase_2))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut nosubs_3))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut optimize_4))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut collate_5))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut ECMAScript_6))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut basic_7))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut extended_8))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut awk_9))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut grep_10))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut egrep_11))) });
}
pub unsafe fn test_12() {
    const _: () = assert!(true, "(BR::icase == std::regex_constants::icase)");;
    const _: () = assert!(true, "(BR::nosubs == std::regex_constants::nosubs)");;
    const _: () = assert!(true, "(BR::optimize == std::regex_constants::optimize)");;
    const _: () = assert!(true, "(BR::collate == std::regex_constants::collate)");;
    const _: () = assert!(true, "(BR::ECMAScript == std::regex_constants::ECMAScript)");;
    const _: () = assert!(true, "(BR::basic == std::regex_constants::basic)");;
    const _: () = assert!(true, "(BR::extended == std::regex_constants::extended)");;
    const _: () = assert!(true, "(BR::awk == std::regex_constants::awk)");;
    const _: () = assert!(true, "(BR::grep == std::regex_constants::grep)");;
    const _: () = assert!(true, "(BR::egrep == std::regex_constants::egrep)");;
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut icase_13))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut nosubs_14))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut optimize_15))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut collate_16))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut ECMAScript_17))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut basic_18))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut extended_19))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut awk_20))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut grep_21))) });
    (unsafe { where_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut egrep_22))) });
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
    (unsafe { test_12() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
