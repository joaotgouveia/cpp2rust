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
    pub unsafe fn eq(mut c1: libc::c_char, mut c2: libc::c_char) -> bool {
        return ((c1 as i32) == (c2 as i32));
    }
    pub unsafe fn lt(mut c1: libc::c_char, mut c2: libc::c_char) -> bool {
        return ((c1 as i32) < (c2 as i32));
    }
    pub unsafe fn compare(
        mut s1: *const libc::c_char,
        mut s2: *const libc::c_char,
        mut n: usize,
    ) -> i32 {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: libc::c_char = (*s1);
                let _c2: libc::c_char = (*s2);
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return -1_i32;
            }
            if (unsafe {
                let _c1: libc::c_char = (*s2);
                let _c2: libc::c_char = (*s1);
                constexpr_char_traits_char_::lt(_c1, _c2)
            }) {
                return 1;
            }
            {
                {
                    n.prefix_dec();
                    s1.prefix_inc()
                };
                s2.prefix_inc()
            };
        }
        return 0;
    }
    pub unsafe fn length(mut s: *const libc::c_char) -> usize {
        let mut len: usize = 0_usize;
        'loop_: while !(unsafe {
            let _c1: libc::c_char = (*s);
            let _c2: libc::c_char = (0 as libc::c_char);
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut s: *const libc::c_char = c"abcde".as_ptr();
        let mut sv0: () = unimplemented!();
        let mut sv1: () = unimplemented!();
        let mut sv2: () = unimplemented!();
        let mut svNot: () = unimplemented!();
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(!(unimplemented!()));
        assert!(unimplemented!());
    }
    {
        let s: *const libc::c_char = c"abcde".as_ptr();
        let sv0: () = unimplemented!();
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        let svNot: () = unimplemented!();
        const _: () = assert!(true, "sv0.starts_with(\"\")");;
        const _: () = assert!(true, "!sv0.starts_with(\"a\")");;
        const _: () = assert!(true, "sv1.starts_with(\"\")");;
        const _: () = assert!(true, "sv1.starts_with(\"a\")");;
        const _: () = assert!(true, "!sv1.starts_with(\"ab\")");;
        const _: () = assert!(true, "!sv1.starts_with(\"abc\")");;
        const _: () = assert!(true, "!sv1.starts_with(\"abcd\")");;
        const _: () = assert!(true, "!sv1.starts_with(\"abcde\")");;
        const _: () = assert!(true, "!sv1.starts_with(\"def\")");;
        const _: () = assert!(true, "sv2.starts_with(s + 5)");;
        const _: () = assert!(true, "sv2.starts_with(\"a\")");;
        const _: () = assert!(true, "sv2.starts_with(\"ab\")");;
        const _: () = assert!(true, "!sv2.starts_with(\"abc\")");;
        const _: () = assert!(true, "!sv2.starts_with(\"abcd\")");;
        const _: () = assert!(true, "!sv2.starts_with(\"abcde\")");;
        const _: () = assert!(true, "!sv2.starts_with(\"def\")");;
        const _: () = assert!(true, "svNot.starts_with(\"\")");;
        const _: () = assert!(true, "!svNot.starts_with(\"a\")");;
        const _: () = assert!(true, "!svNot.starts_with(\"ab\")");;
        const _: () = assert!(true, "!svNot.starts_with(\"abc\")");;
        const _: () = assert!(true, "!svNot.starts_with(\"abcd\")");;
        const _: () = assert!(true, "!svNot.starts_with(\"abcde\")");;
        const _: () = assert!(true, "svNot.starts_with(\"def\")");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
