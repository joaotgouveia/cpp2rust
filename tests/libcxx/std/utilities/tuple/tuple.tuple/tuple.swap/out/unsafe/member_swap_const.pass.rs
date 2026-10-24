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
pub struct ConstSwappable {
    pub i: (),
}
pub unsafe fn swap_0(lhs: *const ConstSwappable, rhs: *const ConstSwappable) {
    unimplemented!();
}
pub unsafe fn test_1() -> () {
    {
        let t0: () = unimplemented!();
        let mut t1: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: () = {
            let mut __tmp_0: ConstSwappable = ConstSwappable { i: 0_() };
            let mut __tmp_1: ConstSwappable = ConstSwappable { i: 1_() };
            std_tuple_ConstSwappable__ConstSwappable_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
        };
        let t1: () = {
            let mut __tmp_2: ConstSwappable = ConstSwappable { i: 2_() };
            let mut __tmp_3: ConstSwappable = ConstSwappable { i: 3_() };
            std_tuple_ConstSwappable__ConstSwappable_::new({ &mut __tmp_2 }, { &mut __tmp_3 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: () = {
            let mut __tmp_4: ConstSwappable = ConstSwappable { i: 0_() };
            let mut __tmp_5: ConstSwappable = ConstSwappable { i: 1_() };
            let mut __tmp_6: ConstSwappable = ConstSwappable { i: 2_() };
            std_tuple_ConstSwappable__const_ConstSwappable__const_ConstSwappable_::new(
                { &mut __tmp_4 },
                { &mut __tmp_5 },
                { &mut __tmp_6 },
            )
        };
        let t1: () = {
            let mut __tmp_7: ConstSwappable = ConstSwappable { i: 3_() };
            let mut __tmp_8: ConstSwappable = ConstSwappable { i: 4_() };
            let mut __tmp_9: ConstSwappable = ConstSwappable { i: 5_() };
            std_tuple_ConstSwappable__const_ConstSwappable__const_ConstSwappable_::new(
                { &mut __tmp_7 },
                { &mut __tmp_8 },
                { &mut __tmp_9 },
            )
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
