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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    let mut l: () = unimplemented!();
    let f32_8: *const () = (unsafe { use_facet_0(unimplemented!()) });
    let f32_16: *const () = (unsafe { use_facet_1(&l) });
    let f16_8: *const () = (unsafe { use_facet_2(unimplemented!()) });
    let mut mbs32_8: () = unimplemented!();
    let mut mbs16_8: () = unimplemented!();
    let mut mbs32_16: () = unimplemented!();
    let mut c32p: *mut () = std::ptr::null_mut();
    let mut c16p: *mut () = std::ptr::null_mut();
    let mut c8p: *mut () = std::ptr::null_mut();
    let mut c_c32p: *const () = std::ptr::null();
    let mut c_c16p: *const () = std::ptr::null();
    let mut c_c8p: *const () = std::ptr::null();
    let mut c32: () = unimplemented!();
    let mut c16: [(); 2] = [unimplemented!(); 2];
    let mut c16c: [(); 4] = [unimplemented!(); 4];
    let mut c16cp: *mut () = std::ptr::null_mut();
    let mut c8: [(); 4] = [unimplemented!(); 4];
    let f32_8t: *const () = (unsafe { use_facet_3(unimplemented!()) });
    let f16_8t: *const () = (unsafe { use_facet_4(unimplemented!()) });
    let mut mbs32_8t: () = unimplemented!();
    let mut mbs16_8t: () = unimplemented!();
    let mut c8tp: *mut () = std::ptr::null_mut();
    let mut c_c8tp: *const () = std::ptr::null();
    let mut c8t: [(); 4] = [unimplemented!(); 4];
    let mut c32x: () = 0_();
    'loop_: while ((c32x) < (1114115_())) {
        if (((55296_()) <= (c32x)) && ((c32x) < (57344_()))) || ((c32x) >= (1114112_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        } else {
            unimplemented!();
            unimplemented!();
            if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            let mut i: () = 0_();
            'loop_: while ((i)
                < ((((c16cp as usize - c16c.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                    as ())
                    / (2_())))
            {
                c16[(i) as usize] = (((c16c[((2_()) * (i)) as usize]) << (8_()))
                    | (c16c[(((2_()) * (i)) + (1_())) as usize]));
                i.prefix_inc();
            }
            c_c16p = (c16.as_mut_ptr().offset(
                ((((c16cp as usize - c16c.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                    as ())
                    / (2_())) as isize,
            ))
            .cast_const();
            unimplemented!();
            if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            if ((c32x) < (128_())) {
                unimplemented!();
            } else if ((c32x) < (2048_())) {
                unimplemented!();
            } else if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            c_c8p = (c8p).cast_const();
            unimplemented!();
            if ((c32x) < (128_())) {
                unimplemented!();
            } else if ((c32x) < (2048_())) {
                unimplemented!();
            } else if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            if ((c32x) < (128_())) {
                unimplemented!();
            } else if ((c32x) < (2048_())) {
                unimplemented!();
            } else if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            c_c8p = (c8p).cast_const();
            unimplemented!();
            if ((c32x) < (128_())) {
                unimplemented!();
            } else if ((c32x) < (2048_())) {
                unimplemented!();
            } else if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            let mut i: () = 0_();
            'loop_: while ((i)
                < (((c16p as usize - c16.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                    as ()))
            {
                c16c[((2_()) * (i)) as usize] = ((c16[(i) as usize]) >> (8_()));
                c16c[(((2_()) * (i)) + (1_())) as usize] = c16[(i) as usize];
                i.prefix_inc();
            }
            let mut c_c16cp: *const () = (c16c.as_mut_ptr().offset(
                ((((c16p as usize - c16.as_mut_ptr() as usize) / ::std::mem::size_of::<()>())
                    as ())
                    * (2_())) as isize,
            ))
            .cast_const();
            unimplemented!();
            if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            if ((c32x) < (128_())) {
                unimplemented!();
            } else if ((c32x) < (2048_())) {
                unimplemented!();
            } else if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            c_c8tp = (c8tp).cast_const();
            unimplemented!();
            if ((c32x) < (128_())) {
                unimplemented!();
            } else if ((c32x) < (2048_())) {
                unimplemented!();
            } else if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            c_c16p = (c16p).cast_const();
            unimplemented!();
            if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            if ((c32x) < (128_())) {
                unimplemented!();
            } else if ((c32x) < (2048_())) {
                unimplemented!();
            } else if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            c_c8tp = (c8tp).cast_const();
            unimplemented!();
            if ((c32x) < (128_())) {
                unimplemented!();
            } else if ((c32x) < (2048_())) {
                unimplemented!();
            } else if ((c32x) < (65536_())) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            unimplemented!();
            unimplemented!();
        }
        c32x.prefix_inc();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
