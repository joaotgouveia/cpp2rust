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
            test_0(&mut _s, ('i' as ()), 0_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 1_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 0_(), 0_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 1_(), 1_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 2_(), 2_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 4_(), 4_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 5_(), 4_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 6_(), 4_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 0_(), 0_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 1_(), 1_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 5_(), 4_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 9_(), 8_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 10_(), 9_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 11_(), 9_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 0_(), 0_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 1_(), 1_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 10_(), 10_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 19_(), 18_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 20_(), 18_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('i' as ()), 21_(), 19_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, ('i' as ()), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, ('i' as ()), 4_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, ('i' as ()), 9_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, ('i' as ()), 19_())
        });
    }
    {
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        const _: () = assert!(true, "sv1.find_last_not_of(\'i\', 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find_last_not_of(\'i\', 1) == SV::npos");;
        const _: () = assert!(true, "sv2.find_last_not_of(\'a\', 0) == SV::npos");;
        const _: () = assert!(true, "sv2.find_last_not_of(\'a\', 1) == 1");;
        const _: () = assert!(true, "sv2.find_last_not_of(\'e\', 5) == 3");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
