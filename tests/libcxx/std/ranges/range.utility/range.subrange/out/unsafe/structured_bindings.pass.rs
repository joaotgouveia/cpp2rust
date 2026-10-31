extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_unreachable_sentinel_t__std_unreachable_sentinel_t_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub unsafe fn test_sized_subrange_2() {
    let mut a: [i32; 4] = [1, 2, 3, 4];
    let mut r: () = unimplemented!();
    assert!(true);
    {
        let mut __decomp_3: () = unimplemented!();
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        assert!(((first) == (a.as_mut_ptr())));
        assert!(((last) == (a.as_mut_ptr().offset((4) as isize))));
    }
    {
        let mut __decomp_4: () = unimplemented!();
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        assert!(((first) == (a.as_mut_ptr())));
        assert!(((last) == (a.as_mut_ptr().offset((4) as isize))));
    }
    {
        let mut __decomp_5: () = unimplemented!();
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        assert!(((first) == (a.as_mut_ptr())));
        assert!(((last) == (a.as_mut_ptr().offset((4) as isize))));
    }
    {
        let mut __decomp_6: () = unimplemented!();
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        assert!(((first) == (a.as_mut_ptr())));
        assert!(((last) == (a.as_mut_ptr().offset((4) as isize))));
    }
}
pub unsafe fn test_unsized_subrange_7() {
    let mut a: [i32; 4] = [1, 2, 3, 4];
    let mut r: () = unimplemented!();
    assert!(!(false));
    {
        let mut __decomp_8: () = unimplemented!();
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        assert!(((first) == (a.as_mut_ptr())));
        const _: () = assert!(true, "");;
    }
    {
        let mut __decomp_9: () = unimplemented!();
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        assert!(((first) == (a.as_mut_ptr())));
        const _: () = assert!(true, "");;
    }
    {
        let mut __decomp_10: () = unimplemented!();
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        assert!(((first) == (a.as_mut_ptr())));
        const _: () = assert!(true, "");;
    }
    {
        let mut __decomp_11: () = unimplemented!();
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        assert!(((first) == (a.as_mut_ptr())));
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn test_copies_not_originals_12() {
    let mut a: [i32; 4] = [1, 2, 3, 4];
    {
        let mut r: () = unimplemented!();
        let __decomp_13: *mut () = &mut r;
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        first = a.as_mut_ptr().offset((2) as isize);
        last = a.as_mut_ptr().offset((2) as isize);
        assert!(((unimplemented!()) == (a.as_mut_ptr())));
        assert!(((unimplemented!()) == (a.as_mut_ptr().offset((4) as isize))));
    }
    {
        let r: () = unimplemented!();
        let __decomp_14: *const () = &r;
        let mut first: () = unimplemented!();
        let mut last: () = unimplemented!();
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        first = a.as_mut_ptr().offset((2) as isize);
        last = a.as_mut_ptr().offset((2) as isize);
        assert!(((unimplemented!()) == (a.as_mut_ptr())));
        assert!(((unimplemented!()) == (a.as_mut_ptr().offset((4) as isize))));
    }
}
pub unsafe fn test_15() -> bool {
    (unsafe { test_sized_subrange_2() });
    (unsafe { test_unsized_subrange_7() });
    (unsafe { test_copies_not_originals_12() });
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
    (unsafe { test_15() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
