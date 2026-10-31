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
    pub unsafe fn find(
        mut s: *const libc::c_char,
        mut n: usize,
        a: *const libc::c_char,
    ) -> *const libc::c_char {
        'loop_: while (n != 0) {
            if (unsafe {
                let _c1: libc::c_char = (*s);
                let _c2: libc::c_char = (*a);
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
    assert!(((unimplemented!()) == (x)));
    if ((x) != (unimplemented!())) {
        assert!(((pos) <= (x)) && (((x).wrapping_add(1_())) <= (unimplemented!())));
    }
}
pub unsafe fn test_1(s: *const (), mut c: (), mut x: ()) {
    assert!(((unimplemented!()) == (x)));
    if ((x) != (unimplemented!())) {
        assert!(((0_()) <= (x)) && (((x).wrapping_add(1_())) <= (unimplemented!())));
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
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(
                &mut _s,
                (('c' as libc::c_char) as ()),
                0_(),
                unimplemented!(),
            )
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(
                &mut _s,
                (('c' as libc::c_char) as ()),
                1_(),
                unimplemented!(),
            )
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, (('c' as libc::c_char) as ()), 0_(), 2_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, (('c' as libc::c_char) as ()), 1_(), 2_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, (('c' as libc::c_char) as ()), 2_(), 2_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(
                &mut _s,
                (('c' as libc::c_char) as ()),
                4_(),
                unimplemented!(),
            )
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(
                &mut _s,
                (('c' as libc::c_char) as ()),
                5_(),
                unimplemented!(),
            )
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(
                &mut _s,
                (('c' as libc::c_char) as ()),
                6_(),
                unimplemented!(),
            )
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, (('c' as libc::c_char) as ()), 0_(), 2_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, (('c' as libc::c_char) as ()), 1_(), 2_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, (('c' as libc::c_char) as ()), 5_(), 7_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(
                &mut _s,
                (('c' as libc::c_char) as ()),
                9_(),
                unimplemented!(),
            )
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(
                &mut _s,
                (('c' as libc::c_char) as ()),
                10_(),
                unimplemented!(),
            )
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(
                &mut _s,
                (('c' as libc::c_char) as ()),
                11_(),
                unimplemented!(),
            )
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, (('c' as libc::c_char) as ()), 0_(), 2_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, (('c' as libc::c_char) as ()), 1_(), 2_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(&mut _s, (('c' as libc::c_char) as ()), 10_(), 12_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(
                &mut _s,
                (('c' as libc::c_char) as ()),
                19_(),
                unimplemented!(),
            )
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(
                &mut _s,
                (('c' as libc::c_char) as ()),
                20_(),
                unimplemented!(),
            )
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_0(
                &mut _s,
                (('c' as libc::c_char) as ()),
                21_(),
                unimplemented!(),
            )
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, (('c' as libc::c_char) as ()), unimplemented!())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, (('c' as libc::c_char) as ()), 2_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, (('c' as libc::c_char) as ()), 2_())
        });
        (unsafe {
            let mut _s: () = unimplemented!();
            test_1(&mut _s, (('c' as libc::c_char) as ()), 2_())
        });
    }
    {
        let sv1: () = unimplemented!();
        let sv2: () = unimplemented!();
        const _: () = assert!(true, "sv1.find(\'c\', 0) == SV::npos");;
        const _: () = assert!(true, "sv1.find(\'c\', 1) == SV::npos");;
        const _: () = assert!(true, "sv2.find(\'c\', 0) == 2");;
        const _: () = assert!(true, "sv2.find(\'c\', 1) == 2");;
        const _: () = assert!(true, "sv2.find(\'c\', 2) == 2");;
        const _: () = assert!(true, "sv2.find(\'c\', 3) == SV::npos");;
        const _: () = assert!(true, "sv2.find(\'c\', 4) == SV::npos");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
