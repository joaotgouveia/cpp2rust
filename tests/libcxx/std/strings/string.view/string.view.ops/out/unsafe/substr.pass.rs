extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test1_0(mut sv: (), mut n: (), mut pos: ()) {
    let mut sv1: () = unimplemented!();
    if ((pos) > (unimplemented!())) {
        return;
    }
    unimplemented!();
    let rlen: () = (*unimplemented!());
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (rlen)) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test1_1(mut sv: (), mut n: (), mut pos: ()) {
    let mut sv1: () = unimplemented!();
    if ((pos) > (unimplemented!())) {
        return;
    }
    unimplemented!();
    let rlen: () = (*unimplemented!());
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (rlen)) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test1_2(mut sv: (), mut n: (), mut pos: ()) {
    let mut sv1: () = unimplemented!();
    if ((pos) > (unimplemented!())) {
        return;
    }
    unimplemented!();
    let rlen: () = (*unimplemented!());
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (rlen)) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test1_3(mut sv: (), mut n: (), mut pos: ()) {
    let mut sv1: () = unimplemented!();
    if ((pos) > (unimplemented!())) {
        return;
    }
    unimplemented!();
    let rlen: () = (*unimplemented!());
    unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (rlen)) {
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_4(mut s: *const ()) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_0(unimplemented!(), 0_(), 0_()) });
    (unsafe { test1_0(unimplemented!(), 1_(), 0_()) });
    (unsafe { test1_0(unimplemented!(), 20_(), 0_()) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_0(_sv, _n, 0_())
    });
    (unsafe { test1_0(unimplemented!(), 0_(), 3_()) });
    (unsafe { test1_0(unimplemented!(), 2_(), 3_()) });
    (unsafe { test1_0(unimplemented!(), 100_(), 3_()) });
    (unsafe {
        test1_0(
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        test1_0(
            unimplemented!(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_0(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_0(_sv, _n, 0_())
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_0(_sv, _n, 1_())
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_0(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_5)),
        )
    });
}
pub unsafe fn test_6(mut s: *const ()) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_1(unimplemented!(), 0_(), 0_()) });
    (unsafe { test1_1(unimplemented!(), 1_(), 0_()) });
    (unsafe { test1_1(unimplemented!(), 20_(), 0_()) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_1(_sv, _n, 0_())
    });
    (unsafe { test1_1(unimplemented!(), 0_(), 3_()) });
    (unsafe { test1_1(unimplemented!(), 2_(), 3_()) });
    (unsafe { test1_1(unimplemented!(), 100_(), 3_()) });
    (unsafe {
        test1_1(
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        test1_1(
            unimplemented!(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_1(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_1(_sv, _n, 0_())
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_1(_sv, _n, 1_())
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_1(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_7)),
        )
    });
}
pub unsafe fn test_8(mut s: *const ()) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_2(unimplemented!(), 0_(), 0_()) });
    (unsafe { test1_2(unimplemented!(), 1_(), 0_()) });
    (unsafe { test1_2(unimplemented!(), 20_(), 0_()) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_2(_sv, _n, 0_())
    });
    (unsafe { test1_2(unimplemented!(), 0_(), 3_()) });
    (unsafe { test1_2(unimplemented!(), 2_(), 3_()) });
    (unsafe { test1_2(unimplemented!(), 100_(), 3_()) });
    (unsafe {
        test1_2(
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_9)),
        )
    });
    (unsafe {
        test1_2(
            unimplemented!(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_9)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_2(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_9)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_2(_sv, _n, 0_())
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_2(_sv, _n, 1_())
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_2(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_9)),
        )
    });
}
pub unsafe fn test_10(mut s: *const ()) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_3(unimplemented!(), 0_(), 0_()) });
    (unsafe { test1_3(unimplemented!(), 1_(), 0_()) });
    (unsafe { test1_3(unimplemented!(), 20_(), 0_()) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_3(_sv, _n, 0_())
    });
    (unsafe { test1_3(unimplemented!(), 0_(), 3_()) });
    (unsafe { test1_3(unimplemented!(), 2_(), 3_()) });
    (unsafe { test1_3(unimplemented!(), 100_(), 3_()) });
    (unsafe {
        test1_3(
            unimplemented!(),
            0_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_11)),
        )
    });
    (unsafe {
        test1_3(
            unimplemented!(),
            2_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_11)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_3(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_11)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_3(_sv, _n, 0_())
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_3(_sv, _n, 1_())
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_3(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_11)),
        )
    });
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
    (unsafe {
        test_4 ( c"ABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDE" .as_ptr() , )
    });
    (unsafe { test_4(c"ABCDE".as_ptr()) });
    (unsafe { test_4(c"a".as_ptr()) });
    (unsafe { test_4(c"".as_ptr()) });
    (unsafe {
        test_6(
            [
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 0 as (),
            ]
            .as_ptr(),
        )
    });
    (unsafe { test_6([65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr()) });
    (unsafe { test_6([97 as (), 0 as ()].as_ptr()) });
    (unsafe { test_6([0 as ()].as_ptr()) });
    (unsafe {
        test_8(
            [
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 0 as (),
            ]
            .as_ptr(),
        )
    });
    (unsafe { test_8([65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr()) });
    (unsafe { test_8([97 as (), 0 as ()].as_ptr()) });
    (unsafe { test_8([0 as ()].as_ptr()) });
    (unsafe {
        test_10(
            [
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (),
                67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (),
                65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (),
                68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 65 as (),
                66 as (), 67 as (), 68 as (), 69 as (), 65 as (), 66 as (), 67 as (), 68 as (),
                69 as (), 0 as (),
            ]
            .as_ptr(),
        )
    });
    (unsafe { test_10([65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr()) });
    (unsafe { test_10([97 as (), 0 as ()].as_ptr()) });
    (unsafe { test_10([0 as ()].as_ptr()) });
    {
        let sv1: () = unimplemented!();
        {
            let sv2: () = unimplemented!();
            const _: () = assert!(true, "sv2.size() == 3");;
            const _: () = assert!(true, "sv2[0] == \'A\'");;
            const _: () = assert!(true, "sv2[1] == \'B\'");;
            const _: () = assert!(true, "sv2[2] == \'C\'");;
        }
        {
            let sv2: () = unimplemented!();
            const _: () = assert!(true, "sv2.size() == 0");;
        }
        {
            let sv2: () = unimplemented!();
            const _: () = assert!(true, "sv2.size() == 2");;
            const _: () = assert!(true, "sv2[0] == \'D\'");;
            const _: () = assert!(true, "sv2[1] == \'E\'");;
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
