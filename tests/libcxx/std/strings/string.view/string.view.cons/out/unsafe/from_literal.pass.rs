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
pub struct constexpr_char_traits_char_ {}
impl constexpr_char_traits_char_ {
    pub unsafe fn eq(mut c1: (), mut c2: ()) -> () {
        return ((c1) == (c2));
    }
    pub unsafe fn length(mut s: *const ()) -> () {
        let mut len: () = 0_();
        'loop_: while !(unsafe {
            let _c1: () = (*s);
            let _c2: () = 0_();
            constexpr_char_traits_char_::eq(_c1, _c2)
        }) {
            len.prefix_inc();
            s.prefix_inc();
        }
        return len;
    }
}
pub unsafe fn StrLen_0(mut s: *const ()) -> () {
    let mut retVal: () = 0_();
    'loop_: while ((*s) != (0_())) {
        retVal.prefix_inc();
        s.prefix_inc();
    }
    return retVal;
}
pub unsafe fn StrLen_1(mut s: *const ()) -> () {
    let mut retVal: () = 0_();
    'loop_: while ((*s) != (0_())) {
        retVal.prefix_inc();
        s.prefix_inc();
    }
    return retVal;
}
pub unsafe fn StrLen_2(mut s: *const ()) -> () {
    let mut retVal: () = 0_();
    'loop_: while ((*s) != (0_())) {
        retVal.prefix_inc();
        s.prefix_inc();
    }
    return retVal;
}
pub unsafe fn StrLen_3(mut s: *const ()) -> () {
    let mut retVal: () = 0_();
    'loop_: while ((*s) != (0_())) {
        retVal.prefix_inc();
        s.prefix_inc();
    }
    return retVal;
}
pub unsafe fn test_4(mut s: *const ()) {
    let mut sv1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_5(mut s: *const ()) {
    let mut sv1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_6(mut s: *const ()) {
    let mut sv1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_7(mut s: *const ()) {
    let mut sv1: () = unimplemented!();
    unimplemented!();
    unimplemented!();
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
    (unsafe { test_4(c"QBCDE".as_ptr()) });
    (unsafe { test_4(c"A".as_ptr()) });
    (unsafe { test_4(c"".as_ptr()) });
    (unsafe { test_5([81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr()) });
    (unsafe { test_5([65 as (), 0 as ()].as_ptr()) });
    (unsafe { test_5([0 as ()].as_ptr()) });
    (unsafe { test_6([81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr()) });
    (unsafe { test_6([65 as (), 0 as ()].as_ptr()) });
    (unsafe { test_6([0 as ()].as_ptr()) });
    (unsafe { test_7([81 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr()) });
    (unsafe { test_7([65 as (), 0 as ()].as_ptr()) });
    (unsafe { test_7([0 as ()].as_ptr()) });
    {
        let sv1: () = unimplemented!();
        const _: () = assert!(true, "sv1.size() == 5");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
