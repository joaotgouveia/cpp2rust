extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
    let mut l: () = unimplemented!();
    let f: *const () = unimplemented!();
    {
        let from: Vec<i32> = std::slice::from_raw_parts(
            [
                115 as i32, 111 as i32, 109 as i32, 101 as i32, 32 as i32, 116 as i32, 101 as i32,
                120 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    115 as i32, 111 as i32, 109 as i32, 101 as i32, 32 as i32, 116 as i32,
                    101 as i32, 120 as i32, 116 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut to: Vec<libc::c_char> = (0..((from.len() - 1).wrapping_add(1_usize)) as usize)
            .map(|_| (0 as libc::c_char))
            .collect::<Vec<_>>();
        let mut mbs: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut from_next: *const () = std::ptr::null();
        let mut to_next: *mut libc::c_char = std::ptr::null_mut();
        let mut r: () = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(
            (((((from_next as usize - from.as_ptr() as usize) / ::std::mem::size_of::<()>()) as i64)
                as usize)
                == (from.len() - 1))
        );
        assert!(
            (((((to_next as usize - to.as_mut_ptr() as usize)
                / ::std::mem::size_of::<libc::c_char>()) as i64) as usize)
                == (from.len() - 1))
        );
        assert!(
            std::slice::from_raw_parts(
                c"some text".as_ptr(),
                libcc2rs::c_str_len(c"some text".as_ptr()) + 1
            )
            .to_vec()
                == std::slice::from_raw_parts(
                    (to.as_mut_ptr()).cast_const(),
                    libcc2rs::c_str_len((to.as_mut_ptr()).cast_const()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut from: Vec<i32> = std::slice::from_raw_parts(
            [
                115 as i32, 111 as i32, 109 as i32, 101 as i32, 32 as i32, 116 as i32, 101 as i32,
                120 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    115 as i32, 111 as i32, 109 as i32, 101 as i32, 32 as i32, 116 as i32,
                    101 as i32, 120 as i32, 116 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        (*(&mut from).as_mut_ptr().add(4_usize as usize)) = (('\0' as libc::c_char) as i32);
        let mut to: Vec<libc::c_char> = (0..((from.len() - 1).wrapping_add(1_usize)) as usize)
            .map(|_| (0 as libc::c_char))
            .collect::<Vec<_>>();
        let mut mbs: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut from_next: *const () = std::ptr::null();
        let mut to_next: *mut libc::c_char = std::ptr::null_mut();
        let mut r: () = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(
            (((((from_next as usize - from.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                as i64) as usize)
                == (from.len() - 1))
        );
        assert!(
            (((((to_next as usize - to.as_mut_ptr() as usize)
                / ::std::mem::size_of::<libc::c_char>()) as i64) as usize)
                == (from.len() - 1))
        );
        assert!(
            (({
                let sa = core::slice::from_raw_parts(
                    (to.as_mut_ptr() as *const ::libc::c_void) as *const u8,
                    (from.len() - 1) as usize,
                );
                let sb = core::slice::from_raw_parts(
                    ((&[
                        (115 as libc::c_char),
                        (111 as libc::c_char),
                        (109 as libc::c_char),
                        (101 as libc::c_char),
                        (0 as libc::c_char),
                        (116 as libc::c_char),
                        (101 as libc::c_char),
                        (120 as libc::c_char),
                        (116 as libc::c_char),
                        (0 as libc::c_char),
                    ])
                        .as_ptr() as *const ::libc::c_void) as *const u8,
                    (from.len() - 1) as usize,
                );
                let mut diff = 0_i32;
                for (x, y) in sa.iter().zip(sb.iter()) {
                    if x != y {
                        diff = (*x as i32) - (*y as i32);
                        break;
                    }
                }
                diff
            }) == (0))
        );
    }
    {
        let mut from: Vec<i32> = std::slice::from_raw_parts(
            [
                115 as i32, 111 as i32, 109 as i32, 101 as i32, 32 as i32, 116 as i32, 101 as i32,
                120 as i32, 116 as i32, 0 as i32,
            ]
            .as_ptr(),
            libcc2rs::c_str_len(
                [
                    115 as i32, 111 as i32, 109 as i32, 101 as i32, 32 as i32, 116 as i32,
                    101 as i32, 120 as i32, 116 as i32, 0 as i32,
                ]
                .as_ptr(),
            ) + 1,
        )
        .to_vec();
        let mut to: Vec<libc::c_char> = (0..((from.len() - 1).wrapping_sub(1_usize)) as usize)
            .map(|_| (0 as libc::c_char))
            .collect::<Vec<_>>();
        let mut mbs: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
        let mut from_next: *const () = std::ptr::null();
        let mut to_next: *mut libc::c_char = std::ptr::null_mut();
        let mut r: () = unimplemented!();
        assert!(((r as i32) == (unimplemented!() as i32)));
        assert!(
            (((((from_next as usize - from.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                as i64) as usize)
                == ((to.len()).wrapping_sub(1_usize)))
        );
        assert!(
            (((((to_next as usize - to.as_mut_ptr() as usize)
                / ::std::mem::size_of::<libc::c_char>()) as i64) as usize)
                == ((to.len()).wrapping_sub(1_usize)))
        );
        assert!(
            std::slice::from_raw_parts(
                c"some te".as_ptr(),
                libcc2rs::c_str_len(c"some te".as_ptr()) + 1
            )
            .to_vec()
                == std::slice::from_raw_parts(
                    (to.as_mut_ptr()).cast_const(),
                    libcc2rs::c_str_len((to.as_mut_ptr()).cast_const()) + 1
                )
                .to_vec()
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
