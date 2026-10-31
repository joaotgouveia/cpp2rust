extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_std_filesystem_path_ref__const_std_filesystem_path_ref_
{}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char_ptr__const_char_ptr_ {}
pub unsafe fn test_constructors_2() {
    {
        let what_arg: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"Hello World".as_ptr(),
            libcc2rs::c_str_len(c"Hello World".as_ptr()) + 1,
        )
        .to_vec();
        let what_contains: Vec<libc::c_char> =
            std::slice::from_raw_parts(unimplemented!(), libcc2rs::c_str_len(unimplemented!()) + 1)
                .to_vec();
        assert!(((unimplemented!()) != (usize::MAX)));
        let mut CheckWhat: FnPtr<fn(*const ())> = lambda_unsafe!(
            {
                let what_contains: Vec<libc::c_char> = what_contains.clone();
            },
            |e: *const ()| {
                let mut s: Vec<libc::c_char> = std::slice::from_raw_parts(
                    unimplemented!(),
                    libcc2rs::c_str_len(unimplemented!()) + 1,
                )
                .to_vec();
                assert!(
                    (({
                        let __hay = &s[..s.len() - 1];
                        let __needle = ::std::slice::from_raw_parts(
                            what_contains.as_ptr(),
                            libcc2rs::c_str_len(what_contains.as_ptr()),
                        );
                        (0_usize..=__hay.len().saturating_sub(__needle.len()))
                            .find(|&__i| __hay[__i..].starts_with(__needle))
                            .unwrap_or(usize::MAX)
                    }) != (usize::MAX))
                );
            }
        );
        let mut ec: () = unimplemented!();
        let p1: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"foo"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        let p2: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"bar"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        {
            const _: () = assert!(true, "");;
            let mut e: () = unimplemented!();
            (unsafe { CheckWhat.call(&e) });
            assert!(unimplemented!());
            assert!((unimplemented!()) && (unimplemented!()));
        }
        {
            const _: () = assert!(true, "");;
            let mut e: () = unimplemented!();
            (unsafe { CheckWhat.call(&e) });
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
        }
        {
            const _: () = assert!(true, "");;
            let mut e: () = unimplemented!();
            (unsafe { CheckWhat.call(&e) });
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
        }
    }
    {
        let what_arg: Vec<libc::c_char> = std::slice::from_raw_parts(
            (&[
                (116 as libc::c_char),
                (101 as libc::c_char),
                (115 as libc::c_char),
                (116 as libc::c_char),
                (32 as libc::c_char),
                (76 as libc::c_char),
                (87 as libc::c_char),
                (71 as libc::c_char),
                (51 as libc::c_char),
                (49 as libc::c_char),
                (49 as libc::c_char),
                (50 as libc::c_char),
                (32 as libc::c_char),
                (119 as libc::c_char),
                (105 as libc::c_char),
                (116 as libc::c_char),
                (104 as libc::c_char),
                (32 as libc::c_char),
                (97 as libc::c_char),
                (110 as libc::c_char),
                (100 as libc::c_char),
                (32 as libc::c_char),
                (87 as libc::c_char),
                (105 as libc::c_char),
                (116 as libc::c_char),
                (104 as libc::c_char),
                (32 as libc::c_char),
                (101 as libc::c_char),
                (109 as libc::c_char),
                (98 as libc::c_char),
                (101 as libc::c_char),
                (100 as libc::c_char),
                (100 as libc::c_char),
                (101 as libc::c_char),
                (100 as libc::c_char),
                (32 as libc::c_char),
                (0 as libc::c_char),
                (32 as libc::c_char),
                (99 as libc::c_char),
                (104 as libc::c_char),
                (97 as libc::c_char),
                (114 as libc::c_char),
                (97 as libc::c_char),
                (99 as libc::c_char),
                (116 as libc::c_char),
                (101 as libc::c_char),
                (114 as libc::c_char),
                (0 as libc::c_char),
            ])
                .as_ptr(),
            libcc2rs::c_str_len(
                (&[
                    (116 as libc::c_char),
                    (101 as libc::c_char),
                    (115 as libc::c_char),
                    (116 as libc::c_char),
                    (32 as libc::c_char),
                    (76 as libc::c_char),
                    (87 as libc::c_char),
                    (71 as libc::c_char),
                    (51 as libc::c_char),
                    (49 as libc::c_char),
                    (49 as libc::c_char),
                    (50 as libc::c_char),
                    (32 as libc::c_char),
                    (119 as libc::c_char),
                    (105 as libc::c_char),
                    (116 as libc::c_char),
                    (104 as libc::c_char),
                    (32 as libc::c_char),
                    (97 as libc::c_char),
                    (110 as libc::c_char),
                    (100 as libc::c_char),
                    (32 as libc::c_char),
                    (87 as libc::c_char),
                    (105 as libc::c_char),
                    (116 as libc::c_char),
                    (104 as libc::c_char),
                    (32 as libc::c_char),
                    (101 as libc::c_char),
                    (109 as libc::c_char),
                    (98 as libc::c_char),
                    (101 as libc::c_char),
                    (100 as libc::c_char),
                    (100 as libc::c_char),
                    (101 as libc::c_char),
                    (100 as libc::c_char),
                    (32 as libc::c_char),
                    (0 as libc::c_char),
                    (32 as libc::c_char),
                    (99 as libc::c_char),
                    (104 as libc::c_char),
                    (97 as libc::c_char),
                    (114 as libc::c_char),
                    (97 as libc::c_char),
                    (99 as libc::c_char),
                    (116 as libc::c_char),
                    (101 as libc::c_char),
                    (114 as libc::c_char),
                    (0 as libc::c_char),
                ])
                    .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let what_contains: Vec<libc::c_char> =
            std::slice::from_raw_parts(unimplemented!(), libcc2rs::c_str_len(unimplemented!()) + 1)
                .to_vec();
        assert!(((unimplemented!()) != (usize::MAX)));
        let mut CheckWhat: FnPtr<fn(*const ())> = lambda_unsafe!(
            {
                let what_contains: Vec<libc::c_char> = what_contains.clone();
            },
            |e: *const ()| {
                let mut s: Vec<libc::c_char> = std::slice::from_raw_parts(
                    unimplemented!(),
                    libcc2rs::c_str_len(unimplemented!()) + 1,
                )
                .to_vec();
                assert!(
                    (({
                        let __hay = &s[..s.len() - 1];
                        let __needle = ::std::slice::from_raw_parts(
                            what_contains.as_ptr(),
                            libcc2rs::c_str_len(what_contains.as_ptr()),
                        );
                        (0_usize..=__hay.len().saturating_sub(__needle.len()))
                            .find(|&__i| __hay[__i..].starts_with(__needle))
                            .unwrap_or(usize::MAX)
                    }) != (usize::MAX))
                );
            }
        );
        let mut ec: () = unimplemented!();
        let p1: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"foo"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        let p2: Vec<libc::c_char> = {
            let mut __v: Vec<libc::c_char> = c"bar"
                .path_bytes()
                .into_iter()
                .map(|__b| __b as libc::c_char)
                .collect();
            __v.push(0);
            __v
        };
        {
            const _: () = assert!(true, "");;
            let mut e: () = unimplemented!();
            (unsafe { CheckWhat.call(&e) });
            assert!(unimplemented!());
            assert!((unimplemented!()) && (unimplemented!()));
        }
        {
            const _: () = assert!(true, "");;
            let mut e: () = unimplemented!();
            (unsafe { CheckWhat.call(&e) });
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
        }
        {
            const _: () = assert!(true, "");;
            let mut e: () = unimplemented!();
            (unsafe { CheckWhat.call(&e) });
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(unimplemented!());
        }
    }
}
pub unsafe fn test_signatures_3() {
    let p: Vec<libc::c_char> = unimplemented!();
    let mut ec: () = unimplemented!();
    let e: () = unimplemented!();
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "");;
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
    const _: () = assert!(
        true,
        "std::is_base_of<std::system_error, fs::filesystem_error>::value"
    );;
    (unsafe { test_constructors_2() });
    (unsafe { test_signatures_3() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
