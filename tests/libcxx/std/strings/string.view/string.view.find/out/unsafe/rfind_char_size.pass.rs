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
}
pub unsafe fn test_0(s: *const (), mut c: (), mut pos: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test_1(s: *const (), mut c: (), mut x: ()) {
    const _: () = assert!(true, "");;
    unimplemented!();
    if ((x) != (unimplemented!())) {
        unimplemented!();
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
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 0_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 1_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 0_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 1_(), 1_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 2_(), 1_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 4_(), 1_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 5_(), 1_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 6_(), 1_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 0_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 1_(), 1_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 5_(), 1_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 9_(), 6_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 10_(), 6_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 11_(), 6_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 0_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 1_(), 1_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 10_(), 6_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 19_(), 16_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 20_(), 16_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('b' as ()), 21_(), 16_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, ('b' as ()), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, ('b' as ()), 1_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, ('b' as ()), 6_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, ('b' as ()), 16_())
        });
    }
    {
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        const _: () = assert!(true, "sv1.rfind(\'b\', 0) == SV::npos");;
        const _: () = assert!(true, "sv1.rfind(\'b\', 1) == SV::npos");;
        const _: () = assert!(true, "sv2.rfind(\'b\', 0) == SV::npos");;
        const _: () = assert!(true, "sv2.rfind(\'b\', 1) == 1");;
        const _: () = assert!(true, "sv2.rfind(\'b\', 2) == 1");;
        const _: () = assert!(true, "sv2.rfind(\'b\', 3) == 1");;
        const _: () = assert!(true, "sv2.rfind(\'b\', 4) == 1");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
