extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test1_0(mut sv: (), mut n: usize, mut pos: usize) {
    let rlen: usize = ({
        let mut __tmp_0: u64 = (n as u64);
        let mut __tmp_1: u64 = (unimplemented!() as u64).wrapping_sub((pos as u64));
        (*{
            let __a: *const _ = &mut __tmp_0;
            let __b: *const _ = &mut __tmp_1;
            if *__b < *__a { __b } else { __a }
        })
    } as usize);
    let mut dest1: *mut libc::c_char = Box::leak(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| (0 as libc::c_char))
            .collect::<Box<[libc::c_char]>>(),
    )
    .as_mut_ptr();
    (*dest1.offset((rlen) as isize)) = (0 as libc::c_char);
    let mut dest2: *mut libc::c_char = Box::leak(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| (0 as libc::c_char))
            .collect::<Box<[libc::c_char]>>(),
    )
    .as_mut_ptr();
    (*dest2.offset((rlen) as isize)) = (0 as libc::c_char);
    if ((pos) > (unimplemented!() as usize)) {
    } else {
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) <= (rlen)) {
            assert!(
                (((*dest1.offset((i) as isize)) as i32) == ((*dest2.offset((i) as isize)) as i32))
            );
            i.prefix_inc();
        }
    }
    {
        let __p = dest1;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<libc::c_char>(),
            )))
        }
    };
    {
        let __p = dest2;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<libc::c_char>(),
            )))
        }
    };
}
pub unsafe fn test1_1(mut sv: (), mut n: usize, mut pos: usize) {
    let rlen: usize = ({
        let mut __tmp_2: u64 = (n as u64);
        let mut __tmp_3: u64 = (unimplemented!() as u64).wrapping_sub((pos as u64));
        (*{
            let __a: *const _ = &mut __tmp_2;
            let __b: *const _ = &mut __tmp_3;
            if *__b < *__a { __b } else { __a }
        })
    } as usize);
    let mut dest1: *mut i32 = Box::leak(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_i32)
            .collect::<Box<[i32]>>(),
    )
    .as_mut_ptr();
    (*dest1.offset((rlen) as isize)) = 0_i32;
    let mut dest2: *mut i32 = Box::leak(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_i32)
            .collect::<Box<[i32]>>(),
    )
    .as_mut_ptr();
    (*dest2.offset((rlen) as isize)) = 0_i32;
    if ((pos) > (unimplemented!() as usize)) {
    } else {
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) <= (rlen)) {
            assert!(((*dest1.offset((i) as isize)) == (*dest2.offset((i) as isize))));
            i.prefix_inc();
        }
    }
    {
        let __p = dest1;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
    {
        let __p = dest2;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<i32>(),
            )))
        }
    };
}
pub unsafe fn test1_2(mut sv: (), mut n: usize, mut pos: usize) {
    let rlen: usize = ({
        let mut __tmp_4: u64 = (n as u64);
        let mut __tmp_5: u64 = (unimplemented!() as u64).wrapping_sub((pos as u64));
        (*{
            let __a: *const _ = &mut __tmp_4;
            let __b: *const _ = &mut __tmp_5;
            if *__b < *__a { __b } else { __a }
        })
    } as usize);
    let mut dest1: *mut u16 = Box::leak(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_u16)
            .collect::<Box<[u16]>>(),
    )
    .as_mut_ptr();
    (*dest1.offset((rlen) as isize)) = 0_u16;
    let mut dest2: *mut u16 = Box::leak(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_u16)
            .collect::<Box<[u16]>>(),
    )
    .as_mut_ptr();
    (*dest2.offset((rlen) as isize)) = 0_u16;
    if ((pos) > (unimplemented!() as usize)) {
    } else {
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) <= (rlen)) {
            assert!(
                (((*dest1.offset((i) as isize)) as i32) == ((*dest2.offset((i) as isize)) as i32))
            );
            i.prefix_inc();
        }
    }
    {
        let __p = dest1;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<u16>(),
            )))
        }
    };
    {
        let __p = dest2;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<u16>(),
            )))
        }
    };
}
pub unsafe fn test1_3(mut sv: (), mut n: usize, mut pos: usize) {
    let rlen: usize = ({
        let mut __tmp_6: u64 = (n as u64);
        let mut __tmp_7: u64 = (unimplemented!() as u64).wrapping_sub((pos as u64));
        (*{
            let __a: *const _ = &mut __tmp_6;
            let __b: *const _ = &mut __tmp_7;
            if *__b < *__a { __b } else { __a }
        })
    } as usize);
    let mut dest1: *mut u32 = Box::leak(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_u32)
            .collect::<Box<[u32]>>(),
    )
    .as_mut_ptr();
    (*dest1.offset((rlen) as isize)) = 0_u32;
    let mut dest2: *mut u32 = Box::leak(
        (0..(rlen).wrapping_add(1_usize))
            .map(|_| 0_u32)
            .collect::<Box<[u32]>>(),
    )
    .as_mut_ptr();
    (*dest2.offset((rlen) as isize)) = 0_u32;
    if ((pos) > (unimplemented!() as usize)) {
    } else {
        unimplemented!();
        unimplemented!();
        let mut i: usize = 0_usize;
        'loop_: while ((i) <= (rlen)) {
            assert!(((*dest1.offset((i) as isize)) == (*dest2.offset((i) as isize))));
            i.prefix_inc();
        }
    }
    {
        let __p = dest1;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<u32>(),
            )))
        }
    };
    {
        let __p = dest2;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<u32>(),
            )))
        }
    };
}
pub unsafe fn test_4(mut s: *const libc::c_char) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_0(unimplemented!(), 0_usize, 0_usize) });
    (unsafe { test1_0(unimplemented!(), 1_usize, 0_usize) });
    (unsafe { test1_0(unimplemented!(), 20_usize, 0_usize) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_0(_sv, _n, 0_usize)
    });
    (unsafe { test1_0(unimplemented!(), 20_usize, (unimplemented!() as usize)) });
    (unsafe { test1_0(unimplemented!(), 0_usize, 3_usize) });
    (unsafe { test1_0(unimplemented!(), 2_usize, 3_usize) });
    (unsafe { test1_0(unimplemented!(), 100_usize, 3_usize) });
    (unsafe { test1_0(unimplemented!(), 100_usize, (unimplemented!() as usize)) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_0(_sv, _n, (unimplemented!() as usize))
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_0(_sv, _n, 0_usize)
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_0(_sv, _n, 1_usize)
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_0(_sv, _n, (unimplemented!() as usize))
    });
}
pub unsafe fn test_5(mut s: *const i32) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_1(unimplemented!(), 0_usize, 0_usize) });
    (unsafe { test1_1(unimplemented!(), 1_usize, 0_usize) });
    (unsafe { test1_1(unimplemented!(), 20_usize, 0_usize) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_1(_sv, _n, 0_usize)
    });
    (unsafe { test1_1(unimplemented!(), 20_usize, (unimplemented!() as usize)) });
    (unsafe { test1_1(unimplemented!(), 0_usize, 3_usize) });
    (unsafe { test1_1(unimplemented!(), 2_usize, 3_usize) });
    (unsafe { test1_1(unimplemented!(), 100_usize, 3_usize) });
    (unsafe { test1_1(unimplemented!(), 100_usize, (unimplemented!() as usize)) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_1(_sv, _n, (unimplemented!() as usize))
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_1(_sv, _n, 0_usize)
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_1(_sv, _n, 1_usize)
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_1(_sv, _n, (unimplemented!() as usize))
    });
}
pub unsafe fn test_6(mut s: *const u16) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_2(unimplemented!(), 0_usize, 0_usize) });
    (unsafe { test1_2(unimplemented!(), 1_usize, 0_usize) });
    (unsafe { test1_2(unimplemented!(), 20_usize, 0_usize) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_2(_sv, _n, 0_usize)
    });
    (unsafe { test1_2(unimplemented!(), 20_usize, (unimplemented!() as usize)) });
    (unsafe { test1_2(unimplemented!(), 0_usize, 3_usize) });
    (unsafe { test1_2(unimplemented!(), 2_usize, 3_usize) });
    (unsafe { test1_2(unimplemented!(), 100_usize, 3_usize) });
    (unsafe { test1_2(unimplemented!(), 100_usize, (unimplemented!() as usize)) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_2(_sv, _n, (unimplemented!() as usize))
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_2(_sv, _n, 0_usize)
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_2(_sv, _n, 1_usize)
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_2(_sv, _n, (unimplemented!() as usize))
    });
}
pub unsafe fn test_7(mut s: *const u32) {
    let mut sv1: () = unimplemented!();
    (unsafe { test1_3(unimplemented!(), 0_usize, 0_usize) });
    (unsafe { test1_3(unimplemented!(), 1_usize, 0_usize) });
    (unsafe { test1_3(unimplemented!(), 20_usize, 0_usize) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_3(_sv, _n, 0_usize)
    });
    (unsafe { test1_3(unimplemented!(), 20_usize, (unimplemented!() as usize)) });
    (unsafe { test1_3(unimplemented!(), 0_usize, 3_usize) });
    (unsafe { test1_3(unimplemented!(), 2_usize, 3_usize) });
    (unsafe { test1_3(unimplemented!(), 100_usize, 3_usize) });
    (unsafe { test1_3(unimplemented!(), 100_usize, (unimplemented!() as usize)) });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = (unimplemented!() as usize);
        test1_3(_sv, _n, (unimplemented!() as usize))
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_3(_sv, _n, 0_usize)
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_3(_sv, _n, 1_usize)
    });
    (unsafe {
        let _sv: () = unimplemented!();
        let _n: usize = ((unimplemented!()).wrapping_add(1_()) as usize);
        test1_3(_sv, _n, (unimplemented!() as usize))
    });
}
pub unsafe fn test_constexpr_copy_8(
    mut abcde: *const libc::c_char,
    mut ghijk: *const libc::c_char,
    mut bcdjk: *const libc::c_char,
) -> bool {
    let mut buf: [libc::c_char; 6] = [
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
        (0 as libc::c_char),
    ];
    let mut lval: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
    return true;
}
pub unsafe fn test_constexpr_copy_9(
    mut abcde: *const i32,
    mut ghijk: *const i32,
    mut bcdjk: *const i32,
) -> bool {
    let mut buf: [i32; 6] = [0_i32, 0_i32, 0_i32, 0_i32, 0_i32, 0_i32];
    let mut lval: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
    return true;
}
pub unsafe fn test_constexpr_copy_10(
    mut abcde: *const u16,
    mut ghijk: *const u16,
    mut bcdjk: *const u16,
) -> bool {
    let mut buf: [u16; 6] = [0_u16, 0_u16, 0_u16, 0_u16, 0_u16, 0_u16];
    let mut lval: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
    return true;
}
pub unsafe fn test_constexpr_copy_11(
    mut abcde: *const u32,
    mut ghijk: *const u32,
    mut bcdjk: *const u32,
) -> bool {
    let mut buf: [u32; 6] = [0_u32, 0_u32, 0_u32, 0_u32, 0_u32, 0_u32];
    let mut lval: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
    return true;
}
pub unsafe fn test_constexpr_copy_12(
    mut abcde: *const u8,
    mut ghijk: *const u8,
    mut bcdjk: *const u8,
) -> bool {
    let mut buf: [u8; 6] = [0_u8, 0_u8, 0_u8, 0_u8, 0_u8, 0_u8];
    let mut lval: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    assert!(unimplemented!());
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe {
        test_4 ( c"ABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDEABCDE" .as_ptr() , )
    });
    (unsafe { test_4(c"ABCDE".as_ptr()) });
    (unsafe { test_4(c"a".as_ptr()) });
    (unsafe { test_4(c"".as_ptr()) });
    (unsafe {
        test_5(
            [
                65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32,
                67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32,
                69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32,
                66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32,
                68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32,
                65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32,
                67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32,
                69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32,
                66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32,
                68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32,
                65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32,
                67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32,
                69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32,
                66 as i32, 67 as i32, 68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32,
                68 as i32, 69 as i32, 65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32,
                0 as i32,
            ]
            .as_ptr(),
        )
    });
    (unsafe {
        test_5(
            [
                65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
            ]
            .as_ptr(),
        )
    });
    (unsafe { test_5([97 as i32, 0 as i32].as_ptr()) });
    (unsafe { test_5([0 as i32].as_ptr()) });
    (unsafe {
        test_6(
            [
                65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16,
                67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16,
                69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16,
                66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16,
                68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16,
                65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16,
                67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16,
                69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16,
                66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16,
                68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16,
                65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16,
                67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16,
                69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16,
                66 as u16, 67 as u16, 68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16,
                68 as u16, 69 as u16, 65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16,
                0 as u16,
            ]
            .as_ptr(),
        )
    });
    (unsafe {
        test_6(
            [
                65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
            ]
            .as_ptr(),
        )
    });
    (unsafe { test_6([97 as u16, 0 as u16].as_ptr()) });
    (unsafe { test_6([0 as u16].as_ptr()) });
    (unsafe {
        test_7(
            [
                65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32,
                67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32,
                69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32,
                66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32,
                68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32,
                65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32,
                67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32,
                69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32,
                66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32,
                68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32,
                65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32,
                67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32,
                69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32,
                66 as u32, 67 as u32, 68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32,
                68 as u32, 69 as u32, 65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32,
                0 as u32,
            ]
            .as_ptr(),
        )
    });
    (unsafe {
        test_7(
            [
                65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
            ]
            .as_ptr(),
        )
    });
    (unsafe { test_7([97 as u32, 0 as u32].as_ptr()) });
    (unsafe { test_7([0 as u32].as_ptr()) });
    (unsafe { test_constexpr_copy_8(c"ABCDE".as_ptr(), c"GHIJK".as_ptr(), c"BCDJK".as_ptr()) });
    (unsafe {
        test_constexpr_copy_9(
            [
                65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
            ]
            .as_ptr(),
            [
                71 as i32, 72 as i32, 73 as i32, 74 as i32, 75 as i32, 0 as i32,
            ]
            .as_ptr(),
            [
                66 as i32, 67 as i32, 68 as i32, 74 as i32, 75 as i32, 0 as i32,
            ]
            .as_ptr(),
        )
    });
    (unsafe {
        test_constexpr_copy_10(
            [
                65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
            ]
            .as_ptr(),
            [
                71 as u16, 72 as u16, 73 as u16, 74 as u16, 75 as u16, 0 as u16,
            ]
            .as_ptr(),
            [
                66 as u16, 67 as u16, 68 as u16, 74 as u16, 75 as u16, 0 as u16,
            ]
            .as_ptr(),
        )
    });
    (unsafe {
        test_constexpr_copy_11(
            [
                65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
            ]
            .as_ptr(),
            [
                71 as u32, 72 as u32, 73 as u32, 74 as u32, 75 as u32, 0 as u32,
            ]
            .as_ptr(),
            [
                66 as u32, 67 as u32, 68 as u32, 74 as u32, 75 as u32, 0 as u32,
            ]
            .as_ptr(),
        )
    });
    (unsafe {
        test_constexpr_copy_12(
            [65 as u8, 66 as u8, 67 as u8, 68 as u8, 69 as u8, 0 as u8].as_ptr(),
            [71 as u8, 72 as u8, 73 as u8, 74 as u8, 75 as u8, 0 as u8].as_ptr(),
            [66 as u8, 67 as u8, 68 as u8, 74 as u8, 75 as u8, 0 as u8].as_ptr(),
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
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
