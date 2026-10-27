extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn check_tag_0(mut _a0: ()) -> () {
    return (unimplemented!()) && (unimplemented!());
}
pub unsafe fn check_tag_1(mut _a0: ()) -> () {
    return (unimplemented!()) && (unimplemented!());
}
pub unsafe fn check_tag_2(mut _a0: ()) -> () {
    return (unimplemented!()) && (unimplemented!());
}
pub unsafe fn check_tag_3(mut _a0: ()) -> () {
    return (unimplemented!()) && (unimplemented!());
}
pub unsafe fn check_tag_4(mut _a0: ()) -> () {
    return (unimplemented!()) && (unimplemented!());
}
pub unsafe fn check_tag_5(mut _a0: ()) -> () {
    return (unimplemented!()) && (unimplemented!());
}
pub unsafe fn check_tag_6(mut _a0: ()) -> () {
    return (unimplemented!()) && (unimplemented!());
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
    {
        const _: () = assert!(true, "check_tag<T>(std::in_place)");;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_same<T1, T2>::value && !std::is_same<T1, T3>::value"
        );;
        const _: () = assert!(true, "!std::is_same<T2, T3>::value");;
        const _: () = assert!(true, "check_tag<T1>(std::in_place_type<void>)");;
        const _: () = assert!(true, "check_tag<T2>(std::in_place_type<int>)");;
        const _: () = assert!(true, "check_tag<T3>(std::in_place_type<const int>)");;
    }
    {
        const _: () = assert!(
            true,
            "!std::is_same<T1, T2>::value && !std::is_same<T1, T3>::value"
        );;
        const _: () = assert!(true, "!std::is_same<T2, T3>::value");;
        const _: () = assert!(true, "check_tag<T1>(std::in_place_index<0>)");;
        const _: () = assert!(true, "check_tag<T2>(std::in_place_index<1>)");;
        const _: () = assert!(
            true,
            "check_tag<T3>(std::in_place_index<static_cast<std::size_t>(-1)>)"
        );;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
