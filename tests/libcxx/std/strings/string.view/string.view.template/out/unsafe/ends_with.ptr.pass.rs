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
    pub unsafe fn lt(mut c1: (), mut c2: ()) -> () {
        return ((c1) < (c2));
    }
    pub unsafe fn compare(mut s1: *const (), mut s2: *const (), mut n: ()) -> () {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: () = (*s1);
                let _c2: () = (*s2);
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return -1_();
            }
            if (unsafe {
                let _c1: () = (*s2);
                let _c2: () = (*s1);
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return 1_();
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0_();
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
        let mut s: *const () = c"abcde".as_ptr();
        let mut sv0: () = unimplemented!();
        let mut sv1: () = unimplemented!();
        let mut sv2: () = unimplemented!();
        let mut svNot: () = unimplemented!();
        const _: () = assert!(true, "");;
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
    {
        let s: *const () = c"abcde".as_ptr();
        let sv0: () = unimplemented!();
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        let svNot: () = unimplemented!();
        const _: () = assert!(true, "sv0.ends_with(\"\")");;
        const _: () = assert!(true, "!sv0.ends_with(\"e\")");;
        const _: () = assert!(true, "sv1.ends_with(\"\")");;
        const _: () = assert!(true, "sv1.ends_with(\"e\")");;
        const _: () = assert!(true, "!sv1.ends_with(\"de\")");;
        const _: () = assert!(true, "!sv1.ends_with(\"cde\")");;
        const _: () = assert!(true, "!sv1.ends_with(\"bcde\")");;
        const _: () = assert!(true, "!sv1.ends_with(\"abcde\")");;
        const _: () = assert!(true, "!sv1.ends_with(\"def\")");;
        const _: () = assert!(true, "sv2.ends_with(\"\")");;
        const _: () = assert!(true, "sv2.ends_with(\"e\")");;
        const _: () = assert!(true, "sv2.ends_with(\"de\")");;
        const _: () = assert!(true, "!sv2.ends_with(\"cde\")");;
        const _: () = assert!(true, "!sv2.ends_with(\"bcde\")");;
        const _: () = assert!(true, "!sv2.ends_with(\"abcde\")");;
        const _: () = assert!(true, "!sv2.ends_with(\"def\")");;
        const _: () = assert!(true, "svNot.ends_with(\"\")");;
        const _: () = assert!(true, "!svNot.ends_with(\"e\")");;
        const _: () = assert!(true, "!svNot.ends_with(\"de\")");;
        const _: () = assert!(true, "!svNot.ends_with(\"cde\")");;
        const _: () = assert!(true, "!svNot.ends_with(\"bcde\")");;
        const _: () = assert!(true, "!svNot.ends_with(\"abcde\")");;
        const _: () = assert!(true, "svNot.ends_with(\"def\")");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
