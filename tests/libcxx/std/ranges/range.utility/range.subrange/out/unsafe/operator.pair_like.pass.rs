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
    "std::convertible_to<std::ranges::subrange<int*>, std::pair<int*, int*>>"
);
const _: () = assert!(
    true,
    "std::convertible_to<std::ranges::subrange<int*>, std::tuple<int*, int*>>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::ranges::subrange<int*>, std::pair<long*, int*>>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::ranges::subrange<int*>, std::pair<int*, long*>>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::ranges::subrange<int*>, std::pair<long*, long*>>"
);
const _: () = assert!(
    !(false),
    "!std::convertible_to<std::ranges::subrange<int*>, std::array<int*, 2>>"
);
pub unsafe fn test_0() -> () {
    {
        let mut data: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let a: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ =
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1({
                &mut data
            });
        {
            let mut p: () = (unsafe { a() });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut p: () = (unsafe { a() });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut p: () = (unsafe { a() });
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut data: [(); 5] = [1_(), 2_(), 3_(), 4_(), 5_()];
        let a: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ =
            std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_::new_1({
                &mut data
            });
        {
            let mut p: () = (unsafe { a() });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut p: () = (unsafe { a() });
            unimplemented!();
            unimplemented!();
        }
        {
            let mut p: () = (unsafe { a() });
            unimplemented!();
            unimplemented!();
        }
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
