extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn truncate_fp_0(mut val: f64) -> f64 {
    let mut sink: f64 = val;
    return sink;
}
pub unsafe fn truncate_fp_1(mut val: f64) -> f64 {
    let mut sink: f64 = val;
    return sink;
}
pub unsafe fn truncate_fp_2(mut val: f32) -> f32 {
    let mut sink: f32 = val;
    return sink;
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
        let mut r: () = unimplemented!();
        let mut f: f32 = unimplemented!();
        assert!(
            ((f) == (unsafe {
                truncate_fp_2(
                    ((((16807_()).wrapping_sub((unimplemented!() as ()))) as f32)
                        / ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                            as f32)
                            + (1_f32))),
                )
            }))
        );
    }
    {
        let mut r: () = unimplemented!();
        let mut f: f32 = unimplemented!();
        assert!(
            ((f) == (unsafe {
                truncate_fp_2(
                    ((((16807_()).wrapping_sub((unimplemented!() as ()))) as f32)
                        / ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                            as f32)
                            + (1_f32))),
                )
            }))
        );
    }
    {
        let mut r: () = unimplemented!();
        let mut f: f32 = unimplemented!();
        assert!(
            ((f) == (unsafe {
                truncate_fp_2(
                    ((((16807_()).wrapping_sub((unimplemented!() as ()))) as f32)
                        / ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                            as f32)
                            + (1_f32))),
                )
            }))
        );
    }
    {
        let mut r: () = unimplemented!();
        let mut f: f32 = unimplemented!();
        assert!(
            ((f) == (unsafe {
                truncate_fp_2(
                    ((((16807_()).wrapping_sub((unimplemented!() as ()))) as f32)
                        / ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                            as f32)
                            + (1_f32))),
                )
            }))
        );
    }
    {
        let mut r: () = unimplemented!();
        let mut f: f32 = unimplemented!();
        assert!(
            ((f) == (unsafe {
                truncate_fp_2(
                    ((((16807_()).wrapping_sub((unimplemented!() as ()))) as f32)
                        / ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                            as f32)
                            + (1_f32))),
                )
            }))
        );
    }
    {
        let mut r: () = unimplemented!();
        let mut f: f64 = unimplemented!();
        assert!(
            ((f) == (unsafe {
                truncate_fp_1(
                    ((((16807_()).wrapping_sub((unimplemented!() as ()))) as f64)
                        / ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                            as f64)
                            + (1_f64))),
                )
            }))
        );
    }
    {
        let mut r: () = unimplemented!();
        let mut f: f64 = unimplemented!();
        assert!(
            ((f) == (unsafe {
                truncate_fp_1(
                    ((((16807_()).wrapping_sub((unimplemented!() as ()))) as f64)
                        / ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                            as f64)
                            + (1_f64))),
                )
            }))
        );
    }
    {
        let mut r: () = unimplemented!();
        let mut f: f64 = unimplemented!();
        assert!(
            ((f) == (unsafe {
                truncate_fp_1(
                    (((((16807_()).wrapping_sub((unimplemented!() as ()))) as f64)
                        + ((((282475249_()).wrapping_sub((unimplemented!() as ()))) as f64)
                            * ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                                as f64)
                                + (1_f64))))
                        / (((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                            as f64)
                            + (1_f64))
                            * ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                                as f64)
                                + (1_f64)))),
                )
            }))
        );
    }
    {
        let mut r: () = unimplemented!();
        let mut f: f64 = unimplemented!();
        assert!(
            ((f) == (unsafe {
                truncate_fp_1(
                    (((((16807_()).wrapping_sub((unimplemented!() as ()))) as f64)
                        + ((((282475249_()).wrapping_sub((unimplemented!() as ()))) as f64)
                            * ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                                as f64)
                                + (1_f64))))
                        / (((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                            as f64)
                            + (1_f64))
                            * ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                                as f64)
                                + (1_f64)))),
                )
            }))
        );
    }
    {
        let mut r: () = unimplemented!();
        let mut f: f64 = unimplemented!();
        assert!(
            ((f) == (unsafe {
                truncate_fp_1(
                    (((((16807_()).wrapping_sub((unimplemented!() as ()))) as f64)
                        + ((((282475249_()).wrapping_sub((unimplemented!() as ()))) as f64)
                            * ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                                as f64)
                                + (1_f64))))
                        / (((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                            as f64)
                            + (1_f64))
                            * ((((unimplemented!() as ()).wrapping_sub((unimplemented!() as ())))
                                as f64)
                                + (1_f64)))),
                )
            }))
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
