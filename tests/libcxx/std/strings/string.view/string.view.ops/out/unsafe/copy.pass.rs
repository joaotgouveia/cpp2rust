extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test1_0(mut sv: (), mut n: (), mut pos: ()) {
    let rlen: () = (*unimplemented!());
    let mut dest1: *mut () = Box::leak(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    )
    .as_mut_ptr();
    (*dest1.offset((rlen) as isize)) = 0_();
    let mut dest2: *mut () = Box::leak(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    )
    .as_mut_ptr();
    (*dest2.offset((rlen) as isize)) = 0_();
    if ((pos) > (unimplemented!())) {
    } else {
        unimplemented!();
        (unsafe { copy_n_1(unimplemented!().offset((pos) as isize), rlen, dest2) });
        let mut i: () = 0_();
        'loop_: while ((i) <= (rlen)) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let __p = dest1;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    {
        let __p = dest2;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
}
pub unsafe fn test1_2(mut sv: (), mut n: (), mut pos: ()) {
    let rlen: () = (*unimplemented!());
    let mut dest1: *mut () = Box::leak(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    )
    .as_mut_ptr();
    (*dest1.offset((rlen) as isize)) = 0_();
    let mut dest2: *mut () = Box::leak(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    )
    .as_mut_ptr();
    (*dest2.offset((rlen) as isize)) = 0_();
    if ((pos) > (unimplemented!())) {
    } else {
        unimplemented!();
        (unsafe { copy_n_3(unimplemented!().offset((pos) as isize), rlen, dest2) });
        let mut i: () = 0_();
        'loop_: while ((i) <= (rlen)) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let __p = dest1;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    {
        let __p = dest2;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
}
pub unsafe fn test1_4(mut sv: (), mut n: (), mut pos: ()) {
    let rlen: () = (*unimplemented!());
    let mut dest1: *mut () = Box::leak(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    )
    .as_mut_ptr();
    (*dest1.offset((rlen) as isize)) = 0_();
    let mut dest2: *mut () = Box::leak(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    )
    .as_mut_ptr();
    (*dest2.offset((rlen) as isize)) = 0_();
    if ((pos) > (unimplemented!())) {
    } else {
        unimplemented!();
        (unsafe { copy_n_5(unimplemented!().offset((pos) as isize), rlen, dest2) });
        let mut i: () = 0_();
        'loop_: while ((i) <= (rlen)) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let __p = dest1;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    {
        let __p = dest2;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
}
pub unsafe fn test1_6(mut sv: (), mut n: (), mut pos: ()) {
    let rlen: () = (*unimplemented!());
    let mut dest1: *mut () = Box::leak(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    )
    .as_mut_ptr();
    (*dest1.offset((rlen) as isize)) = 0_();
    let mut dest2: *mut () = Box::leak(
        (0..(rlen).wrapping_add(1_()))
            .map(|_| unimplemented!())
            .collect::<Box<[()]>>(),
    )
    .as_mut_ptr();
    (*dest2.offset((rlen) as isize)) = 0_();
    if ((pos) > (unimplemented!())) {
    } else {
        unimplemented!();
        (unsafe { copy_n_7(unimplemented!().offset((pos) as isize), rlen, dest2) });
        let mut i: () = 0_();
        'loop_: while ((i) <= (rlen)) {
            unimplemented!();
            i.prefix_inc();
        }
    }
    {
        let __p = dest1;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    {
        let __p = dest2;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
}
pub unsafe fn test_8(mut s: *const ()) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_0(unimplemented!(), 0_(), 0_()) });
    (unsafe { test1_0(unimplemented!(), 1_(), 0_()) });
    (unsafe { test1_0(unimplemented!(), 20_(), 0_()) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_0(_sv, _n, 0_())
    });
    (unsafe {
        test1_0(
            unimplemented!(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_9)),
        )
    });
    (unsafe { test1_0(unimplemented!(), 0_(), 3_()) });
    (unsafe { test1_0(unimplemented!(), 2_(), 3_()) });
    (unsafe { test1_0(unimplemented!(), 100_(), 3_()) });
    (unsafe {
        test1_0(
            unimplemented!(),
            100_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_9)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_0(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_9)),
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
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_9)),
        )
    });
}
pub unsafe fn test_10(mut s: *const ()) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_2(unimplemented!(), 0_(), 0_()) });
    (unsafe { test1_2(unimplemented!(), 1_(), 0_()) });
    (unsafe { test1_2(unimplemented!(), 20_(), 0_()) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_2(_sv, _n, 0_())
    });
    (unsafe {
        test1_2(
            unimplemented!(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_11)),
        )
    });
    (unsafe { test1_2(unimplemented!(), 0_(), 3_()) });
    (unsafe { test1_2(unimplemented!(), 2_(), 3_()) });
    (unsafe { test1_2(unimplemented!(), 100_(), 3_()) });
    (unsafe {
        test1_2(
            unimplemented!(),
            100_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_11)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_2(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_11)),
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
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_11)),
        )
    });
}
pub unsafe fn test_12(mut s: *const ()) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_4(unimplemented!(), 0_(), 0_()) });
    (unsafe { test1_4(unimplemented!(), 1_(), 0_()) });
    (unsafe { test1_4(unimplemented!(), 20_(), 0_()) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_4(_sv, _n, 0_())
    });
    (unsafe {
        test1_4(
            unimplemented!(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_13)),
        )
    });
    (unsafe { test1_4(unimplemented!(), 0_(), 3_()) });
    (unsafe { test1_4(unimplemented!(), 2_(), 3_()) });
    (unsafe { test1_4(unimplemented!(), 100_(), 3_()) });
    (unsafe {
        test1_4(
            unimplemented!(),
            100_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_13)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_4(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_13)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_4(_sv, _n, 0_())
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_4(_sv, _n, 1_())
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_4(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_13)),
        )
    });
}
pub unsafe fn test_14(mut s: *const ()) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_6(unimplemented!(), 0_(), 0_()) });
    (unsafe { test1_6(unimplemented!(), 1_(), 0_()) });
    (unsafe { test1_6(unimplemented!(), 20_(), 0_()) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_6(_sv, _n, 0_())
    });
    (unsafe {
        test1_6(
            unimplemented!(),
            20_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_15)),
        )
    });
    (unsafe { test1_6(unimplemented!(), 0_(), 3_()) });
    (unsafe { test1_6(unimplemented!(), 2_(), 3_()) });
    (unsafe { test1_6(unimplemented!(), 100_(), 3_()) });
    (unsafe {
        test1_6(
            unimplemented!(),
            100_(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_15)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = unimplemented!();
        test1_6(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_15)),
        )
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_6(_sv, _n, 0_())
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_6(_sv, _n, 1_())
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: () = (unimplemented!()).wrapping_add(1_());
        test1_6(
            _sv,
            _n,
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_15)),
        )
    });
}
pub unsafe fn test_constexpr_copy_16(
    mut abcde: *const (),
    mut ghijk: *const (),
    mut bcdjk: *const (),
) -> () {
    let mut buf: [(); 6] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut lval: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_constexpr_copy_17(
    mut abcde: *const (),
    mut ghijk: *const (),
    mut bcdjk: *const (),
) -> () {
    let mut buf: [(); 6] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut lval: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_constexpr_copy_18(
    mut abcde: *const (),
    mut ghijk: *const (),
    mut bcdjk: *const (),
) -> () {
    let mut buf: [(); 6] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut lval: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_constexpr_copy_19(
    mut abcde: *const (),
    mut ghijk: *const (),
    mut bcdjk: *const (),
) -> () {
    let mut buf: [(); 6] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut lval: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_constexpr_copy_20(
    mut abcde: *const (),
    mut ghijk: *const (),
    mut bcdjk: *const (),
) -> () {
    let mut buf: [(); 6] = [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ];
    let mut lval: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
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
        test_8 ( c"ABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDE" .as_ptr() , )
    });
    (unsafe { test_8(c"ABCDE".as_ptr()) });
    (unsafe { test_8(c"a".as_ptr()) });
    (unsafe { test_8(c"".as_ptr()) });
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
    (unsafe {
        test_12(
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
    (unsafe { test_12([65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr()) });
    (unsafe { test_12([97 as (), 0 as ()].as_ptr()) });
    (unsafe { test_12([0 as ()].as_ptr()) });
    (unsafe {
        test_14(
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
    (unsafe { test_14([65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr()) });
    (unsafe { test_14([97 as (), 0 as ()].as_ptr()) });
    (unsafe { test_14([0 as ()].as_ptr()) });
    (unsafe { test_constexpr_copy_16(c"ABCDE".as_ptr(), c"GHIJK".as_ptr(), c"BCDJK".as_ptr()) });
    (unsafe {
        test_constexpr_copy_17(
            [65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr(),
            [71 as (), 72 as (), 73 as (), 74 as (), 75 as (), 0 as ()].as_ptr(),
            [66 as (), 67 as (), 68 as (), 74 as (), 75 as (), 0 as ()].as_ptr(),
        )
    });
    (unsafe {
        test_constexpr_copy_18(
            [65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr(),
            [71 as (), 72 as (), 73 as (), 74 as (), 75 as (), 0 as ()].as_ptr(),
            [66 as (), 67 as (), 68 as (), 74 as (), 75 as (), 0 as ()].as_ptr(),
        )
    });
    (unsafe {
        test_constexpr_copy_19(
            [65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr(),
            [71 as (), 72 as (), 73 as (), 74 as (), 75 as (), 0 as ()].as_ptr(),
            [66 as (), 67 as (), 68 as (), 74 as (), 75 as (), 0 as ()].as_ptr(),
        )
    });
    (unsafe {
        test_constexpr_copy_20(
            [65 as (), 66 as (), 67 as (), 68 as (), 69 as (), 0 as ()].as_ptr(),
            [71 as (), 72 as (), 73 as (), 74 as (), 75 as (), 0 as ()].as_ptr(),
            [66 as (), 67 as (), 68 as (), 74 as (), 75 as (), 0 as ()].as_ptr(),
        )
    });
    const _: () = assert!(true, "test_constexpr_copy(\"ABCDE\", \"GHIJK\", \"BCDJK\")");;
    const _: () = assert!(
        true,
        "test_constexpr_copy(L\"ABCDE\", L\"GHIJK\", L\"BCDJK\")"
    );;
    const _: () = assert!(
        true,
        "test_constexpr_copy(u\"ABCDE\", u\"GHIJK\", u\"BCDJK\")"
    );;
    const _: () = assert!(
        true,
        "test_constexpr_copy(U\"ABCDE\", U\"GHIJK\", U\"BCDJK\")"
    );;
    const _: () = assert!(
        true,
        "test_constexpr_copy(u8\"ABCDE\", u8\"GHIJK\", u8\"BCDJK\")"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
