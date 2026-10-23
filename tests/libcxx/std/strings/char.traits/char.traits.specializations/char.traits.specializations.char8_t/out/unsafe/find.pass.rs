extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_constexpr_0() -> () {
    let p: *const () = [49 as (), 50 as (), 51 as (), 0 as ()].as_ptr();
    return ((((unsafe {
        let ___s: *const () = p;
        let mut ___a: () = (49 as ());
        std_char_traits_char8_t_::find(___s, 3_(), &mut ___a)
    }) == (p))
        && ((unsafe {
            let ___s: *const () = p;
            let mut ___a: () = (50 as ());
            std_char_traits_char8_t_::find(___s, 3_(), &mut ___a)
        }) == (p.offset((1_()) as isize))))
        && ((unsafe {
            let ___s: *const () = p;
            let mut ___a: () = (51 as ());
            std_char_traits_char8_t_::find(___s, 3_(), &mut ___a)
        }) == (p.offset((2_()) as isize))))
        && ((unsafe {
            let ___s: *const () = p;
            let mut ___a: () = (52 as ());
            std_char_traits_char8_t_::find(___s, 3_(), &mut ___a)
        })
        .is_null());
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
    let mut s1: [(); 3] = [1_(), 2_(), 3_()];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    const _: () = assert!(true, "test_constexpr()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
