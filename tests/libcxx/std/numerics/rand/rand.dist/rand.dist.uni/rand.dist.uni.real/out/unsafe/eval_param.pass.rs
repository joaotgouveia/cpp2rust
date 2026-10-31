extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sqr_0(mut x: f64) -> f64 {
    return ((x) * (x));
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
        let mut g: () = unimplemented!();
        let mut d: () = unimplemented!();
        let mut p: () = unimplemented!();
        let N: i32 = 100000;
        let mut u: Vec<f64> = Vec::new();
        let mut i: i32 = 0;
        'loop_: while ((i) < (N)) {
            let mut v: () = unimplemented!();
            assert!(((unimplemented!()) <= (v)) && ((v) < (unimplemented!())));
            {
                let a0_clone = v.clone();
                u.push(a0_clone)
            };
            i.prefix_inc();
        }
        let mut mean: () = ((unimplemented!()) / (u.len() as f64));
        let mut var: () = 0_();
        let mut skew: () = 0_();
        let mut kurtosis: () = 0_();
        let mut i: usize = 0_usize;
        'loop_: while ((i) < (u.len())) {
            let mut dbl: () = ((*(&mut u).as_mut_ptr().add(i as usize)) - (mean));
            let mut d2: () = (unsafe { sqr_0(dbl) });
            var += d2;
            skew += ((dbl) * (d2));
            kurtosis += ((d2) * (d2));
            i.prefix_inc();
        }
        var /= (u.len() as ());
        let mut dev: () = unimplemented!();
        skew /= (((u.len() as ()) * (dev)) * (var));
        kurtosis /= (((u.len() as ()) * (var)) * (var));
        kurtosis -= 3_();
        let mut x_mean: () = (((unimplemented!()) + (unimplemented!())) / (2_()));
        let mut x_var: () =
            ((unsafe { sqr_0(((unimplemented!()) - (unimplemented!()))) }) / (12_f64));
        let mut x_skew: () = 0_();
        let mut x_kurtosis: () = ((-6_f64) / (5_f64));
        assert!(((unimplemented!()) < (0.01_f64)));
        assert!(((unimplemented!()) < (0.01_f64)));
        assert!(((unimplemented!()) < (0.01_f64)));
        assert!(((unimplemented!()) < (0.01_f64)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
