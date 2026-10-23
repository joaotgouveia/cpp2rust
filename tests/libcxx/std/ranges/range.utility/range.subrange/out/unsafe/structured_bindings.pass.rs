extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_std_unreachable_sentinel_t__std_unreachable_sentinel_t_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ptr__int_ptr_ {}
pub unsafe fn test_sized_subrange_2() {
    let mut a: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    let mut r: () = unimplemented!();
    unimplemented!();
    {
        let mut __decomp_3: () = unimplemented!();
        let mut first: () = (unsafe { get_4(&mut __decomp_3) });
        let mut last: () = (unsafe { get_5(&mut __decomp_3) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut __decomp_6: std_ranges_subrange_int_ptr__int_ptr__std_ranges_subrange_kind_sized_ =
            unimplemented!();
        let mut first: () = (unsafe { get_4(&mut *(&raw mut __decomp_6).cast::<()>()) });
        let mut last: () = (unsafe { get_5(&mut *(&raw mut __decomp_6).cast::<()>()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut __decomp_7: () = unimplemented!();
        let mut first: () = (unsafe { get_4(&mut __decomp_7) });
        let mut last: () = (unsafe { get_5(&mut __decomp_7) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut __decomp_8: () = unimplemented!();
        let mut first: () = (unsafe { get_4(&mut __decomp_8) });
        let mut last: () = (unsafe { get_5(&mut __decomp_8) });
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_unsized_subrange_9() {
    let mut a: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    let mut r: () = unimplemented!();
    unimplemented!();
    {
        let mut __decomp_10: () = unimplemented!();
        let mut first: () = (unsafe { get_11(&mut __decomp_10) });
        let mut last: () = (unsafe { get_12(&mut __decomp_10) });
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut __decomp_13 : std_ranges_subrange_int_ptr__std_unreachable_sentinel_t__std_ranges_subrange_kind_unsized_ = unimplemented!() ;
        let mut first: () = (unsafe { get_11(&mut *(&raw mut __decomp_13).cast::<()>()) });
        let mut last: () = (unsafe { get_12(&mut *(&raw mut __decomp_13).cast::<()>()) });
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut __decomp_14: () = unimplemented!();
        let mut first: () = (unsafe { get_11(&mut __decomp_14) });
        let mut last: () = (unsafe { get_12(&mut __decomp_14) });
        unimplemented!();
        const _: () = assert!(true, "");;
    }
    {
        let mut __decomp_15: () = unimplemented!();
        let mut first: () = (unsafe { get_11(&mut __decomp_15) });
        let mut last: () = (unsafe { get_12(&mut __decomp_15) });
        unimplemented!();
        const _: () = assert!(true, "");;
    }
}
pub unsafe fn test_copies_not_originals_16() {
    let mut a: [(); 4] = [1_(), 2_(), 3_(), 4_()];
    {
        let mut r: () = unimplemented!();
        let __decomp_17: *mut () = &mut r;
        let mut first: () = (unsafe { get_18(__decomp_17) });
        let mut last: () = (unsafe { get_19(__decomp_17) });
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        first = a.as_mut_ptr().offset((2_()) as isize);
        last = a.as_mut_ptr().offset((2_()) as isize);
        unimplemented!();
        unimplemented!();
    }
    {
        let r: () = unimplemented!();
        let __decomp_20: *const () = &r;
        let mut first: () = (unsafe { get_18(__decomp_20) });
        let mut last: () = (unsafe { get_19(__decomp_20) });
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
        first = a.as_mut_ptr().offset((2_()) as isize);
        last = a.as_mut_ptr().offset((2_()) as isize);
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_21() -> () {
    (unsafe { test_sized_subrange_2() });
    (unsafe { test_unsized_subrange_9() });
    (unsafe { test_copies_not_originals_16() });
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
    (unsafe { test_21() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
