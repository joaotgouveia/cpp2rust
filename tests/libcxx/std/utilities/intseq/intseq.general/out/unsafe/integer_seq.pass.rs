extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn extract_0(t: *const (), _a1: ()) -> () {
    return unimplemented!();
}
pub unsafe fn extract_1(t: *const (), _a1: ()) -> () {
    return unimplemented!();
}
pub unsafe fn extract_2(t: *const (), _a1: ()) -> () {
    return unimplemented!();
}
pub unsafe fn extract_3(t: *const (), _a1: ()) -> () {
    return unimplemented!();
}
pub unsafe fn extract_4(t: *const (), _a1: ()) -> () {
    return unimplemented!();
}
pub unsafe fn extract_5(t: *const (), _a1: ()) -> () {
    return unimplemented!();
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
    const _: () = assert!(true, "std::is_same<int3::value_type, int>::value");;
    const _: () = assert!(true, "int3::size () == 3");;
    const _: () = assert!(true, "std::is_same<size7::value_type, std::size_t>::value");;
    const _: () = assert!(true, "size7::size () == 7");;
    const _: () = assert!(true, "std::is_same<size4::value_type, std::size_t>::value");;
    const _: () = assert!(true, "size4::size () == 4");;
    const _: () = assert!(true, "std::is_same<size2::value_type, std::size_t>::value");;
    const _: () = assert!(true, "size2::size () == 2");;
    const _: () = assert!(true, "std::is_same<intmix::value_type, int>::value");;
    const _: () = assert!(true, "intmix::size () == 4");;
    const _: () = assert!(
        true,
        "std::is_same<sizemix::value_type, std::size_t>::value"
    );;
    const _: () = assert!(true, "sizemix::size () == 5");;
    let mut tup: () = unimplemented!();
    let mut t3: () = (unsafe { extract_0(&tup, unimplemented!()) });
    const _: () = assert!(
        true,
        "std::tuple_size<decltype(t3)>::value == int3::size ()"
    );;
    unimplemented!();
    let mut t7: () = (unsafe { extract_1(&tup, unimplemented!()) });
    const _: () = assert!(
        true,
        "std::tuple_size<decltype(t7)>::value == size7::size ()"
    );;
    unimplemented!();
    let mut t4: () = (unsafe { extract_2(&tup, unimplemented!()) });
    const _: () = assert!(
        true,
        "std::tuple_size<decltype(t4)>::value == size4::size ()"
    );;
    unimplemented!();
    let mut t2: () = (unsafe { extract_3(&tup, unimplemented!()) });
    const _: () = assert!(
        true,
        "std::tuple_size<decltype(t2)>::value == size2::size ()"
    );;
    unimplemented!();
    let mut tintmix: () = (unsafe { extract_4(&tup, unimplemented!()) });
    const _: () = assert!(
        true,
        "std::tuple_size<decltype(tintmix)>::value == intmix::size ()"
    );;
    unimplemented!();
    let mut tsizemix: () = (unsafe { extract_5(&tup, unimplemented!()) });
    const _: () = assert!(
        true,
        "std::tuple_size<decltype(tsizemix)>::value == sizemix::size ()"
    );;
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
