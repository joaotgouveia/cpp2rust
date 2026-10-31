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
    let f32_8: *const () = unimplemented!();
    let f32_16: *const () = unimplemented!();
    let f16_8: *const () = unimplemented!();
    let mut mbs32_8: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
    let mut mbs16_8: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
    let mut mbs32_16: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
    let mut c32p: *mut () = std::ptr::null_mut();
    let mut c16p: *mut () = std::ptr::null_mut();
    let mut c8p: *mut () = std::ptr::null_mut();
    let mut c_c32p: *const () = std::ptr::null();
    let mut c_c16p: *const () = std::ptr::null();
    let mut c_c8p: *const () = std::ptr::null();
    let mut c32: () = unimplemented!();
    let mut c16: [(); 2] = [unimplemented!(); 2];
    let mut c16c: [libc::c_char; 4] = [(0 as libc::c_char); 4];
    let mut c16cp: *mut libc::c_char = std::ptr::null_mut();
    let mut c8: [(); 4] = [unimplemented!(); 4];
    let f32_8t: *const () = unimplemented!();
    let f16_8t: *const () = unimplemented!();
    let mut mbs32_8t: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
    let mut mbs16_8t: ::libc::mbstate_t = unsafe { std::mem::zeroed() };
    let mut c8tp: *mut () = std::ptr::null_mut();
    let mut c_c8tp: *const () = std::ptr::null();
    let mut c8t: [(); 4] = [unimplemented!(); 4];
    let mut c32x: () = 0_();
    'loop_: while ((c32x as u32) < (1114115_u32)) {
        if (((55296_u32) <= (c32x as u32)) && ((c32x as u32) < (57344_u32)))
            || ((c32x as u32) >= (1114112_u32))
        {
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
        } else {
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!(
                ((((c_c32p as usize - (&mut c32x as *mut ()) as usize)
                    / ::std::mem::size_of::<()>()) as i64)
                    == (1_i64))
            );
            if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c16cp as usize - c16c.as_mut_ptr() as usize)
                        / ::std::mem::size_of::<libc::c_char>()) as i64)
                        == (2_i64))
                );
            } else {
                assert!(
                    ((((c16cp as usize - c16c.as_mut_ptr() as usize)
                        / ::std::mem::size_of::<libc::c_char>()) as i64)
                        == (4_i64))
                );
            }
            let mut i: i32 = 0;
            'loop_: while ((i as i64)
                < ((((c16cp as usize - c16c.as_mut_ptr() as usize)
                    / ::std::mem::size_of::<libc::c_char>()) as i64)
                    / (2_i64)))
            {
                c16[(i) as usize] = ((((((c16c[((2) * (i)) as usize] as u8) as i32) << (8))
                    | ((c16c[(((2) * (i)) + (1)) as usize] as u8) as i32))
                    as u16) as ());
                i.prefix_inc();
            }
            c_c16p = (c16.as_mut_ptr().offset(
                ((((c16cp as usize - c16c.as_mut_ptr() as usize)
                    / ::std::mem::size_of::<libc::c_char>()) as i64)
                    / (2_i64)) as isize,
            ))
            .cast_const();
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c_c16p as usize - c16.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (1_i64))
                );
            } else {
                assert!(
                    ((((c_c16p as usize - c16.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (2_i64))
                );
            }
            if ((c32x as u32) < (128_u32)) {
                assert!(
                    ((((c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (1_i64))
                );
            } else if ((c32x as u32) < (2048_u32)) {
                assert!(
                    ((((c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (2_i64))
                );
            } else if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (3_i64))
                );
            } else {
                assert!(
                    ((((c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (4_i64))
                );
            }
            c_c8p = (c8p).cast_const();
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if ((c32x as u32) < (128_u32)) {
                assert!(
                    ((((c_c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (1_i64))
                );
            } else if ((c32x as u32) < (2048_u32)) {
                assert!(
                    ((((c_c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (2_i64))
                );
            } else if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c_c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (3_i64))
                );
            } else {
                assert!(
                    ((((c_c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (4_i64))
                );
            }
            assert!(
                ((((c32p as usize - (&mut c32 as *mut ()) as usize) / ::std::mem::size_of::<()>())
                    as i64)
                    == (1_i64))
            );
            assert!(((c32 as u32) == (c32x as u32)));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!(
                ((((c_c32p as usize - (&mut c32x as *mut ()) as usize)
                    / ::std::mem::size_of::<()>()) as i64)
                    == (1_i64))
            );
            if ((c32x as u32) < (128_u32)) {
                assert!(
                    ((((c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (1_i64))
                );
            } else if ((c32x as u32) < (2048_u32)) {
                assert!(
                    ((((c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (2_i64))
                );
            } else if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (3_i64))
                );
            } else {
                assert!(
                    ((((c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (4_i64))
                );
            }
            c_c8p = (c8p).cast_const();
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if ((c32x as u32) < (128_u32)) {
                assert!(
                    ((((c_c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (1_i64))
                );
            } else if ((c32x as u32) < (2048_u32)) {
                assert!(
                    ((((c_c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (2_i64))
                );
            } else if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c_c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (3_i64))
                );
            } else {
                assert!(
                    ((((c_c8p as usize - c8.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (4_i64))
                );
            }
            if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c16p as usize - c16.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (1_i64))
                );
            } else {
                assert!(
                    ((((c16p as usize - c16.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (2_i64))
                );
            }
            let mut i: i32 = 0;
            'loop_: while ((i as i64)
                < (((c16p as usize - c16.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                    as i64))
            {
                c16c[((2) * (i)) as usize] = (((c16[(i) as usize] as i32) >> (8)) as libc::c_char);
                c16c[(((2) * (i)) + (1)) as usize] = (c16[(i) as usize] as libc::c_char);
                i.prefix_inc();
            }
            let mut c_c16cp: *const libc::c_char = (c16c.as_mut_ptr().offset(
                ((((c16p as usize - c16.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                    as i64)
                    * (2_i64)) as isize,
            ))
            .cast_const();
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c_c16cp as usize - c16c.as_mut_ptr() as usize)
                        / ::std::mem::size_of::<libc::c_char>()) as i64)
                        == (2_i64))
                );
            } else {
                assert!(
                    ((((c_c16cp as usize - c16c.as_mut_ptr() as usize)
                        / ::std::mem::size_of::<libc::c_char>()) as i64)
                        == (4_i64))
                );
            }
            assert!(
                ((((c32p as usize - (&mut c32 as *mut ()) as usize) / ::std::mem::size_of::<()>())
                    as i64)
                    == (1_i64))
            );
            assert!(((c32 as u32) == (c32x as u32)));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            assert!(
                ((((c_c32p as usize - (&mut c32x as *mut ()) as usize)
                    / ::std::mem::size_of::<()>()) as i64)
                    == (1_i64))
            );
            if ((c32x as u32) < (128_u32)) {
                assert!(
                    ((((c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (1_i64))
                );
            } else if ((c32x as u32) < (2048_u32)) {
                assert!(
                    ((((c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (2_i64))
                );
            } else if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (3_i64))
                );
            } else {
                assert!(
                    ((((c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (4_i64))
                );
            }
            c_c8tp = (c8tp).cast_const();
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if ((c32x as u32) < (128_u32)) {
                assert!(
                    ((((c_c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (1_i64))
                );
            } else if ((c32x as u32) < (2048_u32)) {
                assert!(
                    ((((c_c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (2_i64))
                );
            } else if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c_c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (3_i64))
                );
            } else {
                assert!(
                    ((((c_c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (4_i64))
                );
            }
            if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c16p as usize - c16.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (1_i64))
                );
            } else {
                assert!(
                    ((((c16p as usize - c16.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (2_i64))
                );
            }
            c_c16p = (c16p).cast_const();
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c_c16p as usize - c16.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (1_i64))
                );
            } else {
                assert!(
                    ((((c_c16p as usize - c16.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (2_i64))
                );
            }
            if ((c32x as u32) < (128_u32)) {
                assert!(
                    ((((c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (1_i64))
                );
            } else if ((c32x as u32) < (2048_u32)) {
                assert!(
                    ((((c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (2_i64))
                );
            } else if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (3_i64))
                );
            } else {
                assert!(
                    ((((c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (4_i64))
                );
            }
            c_c8tp = (c8tp).cast_const();
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
            if ((c32x as u32) < (128_u32)) {
                assert!(
                    ((((c_c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (1_i64))
                );
            } else if ((c32x as u32) < (2048_u32)) {
                assert!(
                    ((((c_c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (2_i64))
                );
            } else if ((c32x as u32) < (65536_u32)) {
                assert!(
                    ((((c_c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (3_i64))
                );
            } else {
                assert!(
                    ((((c_c8tp as usize - c8t.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                        as i64)
                        == (4_i64))
                );
            }
            assert!(
                ((((c32p as usize - (&mut c32 as *mut ()) as usize) / ::std::mem::size_of::<()>())
                    as i64)
                    == (1_i64))
            );
            assert!(((c32 as u32) == (c32x as u32)));
        }
        c32x.prefix_inc();
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
