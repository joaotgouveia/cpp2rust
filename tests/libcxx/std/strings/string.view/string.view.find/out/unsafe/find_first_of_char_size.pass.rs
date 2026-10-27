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
    pub unsafe fn find(mut s: *const (), mut n: (), a: *const ()) -> *const () {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: () = (*s);
                let _c2: () = (*a);
                constexpr_char_traits_char_::eq(_c1, _c2)
            }) {
                return s;
            }
            s.prefix_inc();
            n.prefix_dec();
        }
        return std::ptr::null();
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
            test_0(&mut _s, ('e' as ()), 0_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 1_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 0_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 1_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 2_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 4_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 5_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 6_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 0_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 1_(), 4_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 5_(), 6_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 9_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 10_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 11_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 0_(), 7_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 1_(), 4_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 10_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 19_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 20_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, ('e' as ()), 21_(), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, ('e' as ()), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, ('e' as ()), 4_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, ('e' as ()), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, ('e' as ()), 2_())
        });
    }
    {
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        const _: () = assert!(true, "sv1.find_first_of(\'e\', 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find_first_of(\'e\', 1) == SV::npos");;
        const _: () = assert!(true, "sv2.find_first_of(\'q\', 0) == SV::npos");;
        const _: () = assert!(true, "sv2.find_first_of(\'e\', 1) == 4");;
        const _: () = assert!(true, "sv2.find_first_of(\'e\', 5) == SV::npos");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
