extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn sqr_0(mut x: ()) -> () {
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut g: () = unimplemented!();
        let mut d: () = unimplemented!();
        let mut p: () = unimplemented!();
        let N: () = 100000_();
        let mut u: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (N)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut mean: () = ((unimplemented!()) / (unimplemented!()));
        let mut var: () = 0_();
        let mut skew: () = 0_();
        let mut kurtosis: () = 0_();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            let mut dbl: () = ((unimplemented!()) - (mean));
            let mut d2: () = (unsafe { sqr_0(dbl) });
            var += d2;
            skew += ((dbl) * (d2));
            kurtosis += ((d2) * (d2));
            i.prefix_inc();
        }
        var /= unimplemented!();
        let mut dev: () = (unsafe { sqrt_1(var) });
        skew /= (((unimplemented!()) * (dev)) * (var));
        kurtosis /= (((unimplemented!()) * (var)) * (var));
        kurtosis -= 3_();
        let mut x_mean: () = unimplemented!();
        let mut x_var: () = ((unimplemented!()) * ((1_()) - (unimplemented!())));
        let mut x_skew: () =
            (((1_()) - ((2_()) * (unimplemented!()))) / (unsafe { sqrt_1(x_var) }));
        let mut x_kurtosis: () = (((((6_()) * (unsafe { sqr_0(unimplemented!()) }))
            - ((6_()) * (unimplemented!())))
            + (1_()))
            / (x_var));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut g: () = unimplemented!();
        let mut d: () = unimplemented!();
        let mut p: () = unimplemented!();
        let N: () = 100000_();
        let mut u: () = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) < (N)) {
            unimplemented!();
            i.prefix_inc();
        }
        let mut mean: () = ((unimplemented!()) / (unimplemented!()));
        let mut var: () = 0_();
        let mut skew: () = 0_();
        let mut kurtosis: () = 0_();
        let mut i: () = 0_();
        'loop_: while ((i) < (unimplemented!())) {
            let mut dbl: () = ((unimplemented!()) - (mean));
            let mut d2: () = (unsafe { sqr_0(dbl) });
            var += d2;
            skew += ((dbl) * (d2));
            kurtosis += ((d2) * (d2));
            i.prefix_inc();
        }
        var /= unimplemented!();
        let mut dev: () = (unsafe { sqrt_1(var) });
        skew /= (((unimplemented!()) * (dev)) * (var));
        kurtosis /= (((unimplemented!()) * (var)) * (var));
        kurtosis -= 3_();
        let mut x_mean: () = unimplemented!();
        let mut x_var: () = ((unimplemented!()) * ((1_()) - (unimplemented!())));
        let mut x_skew: () =
            (((1_()) - ((2_()) * (unimplemented!()))) / (unsafe { sqrt_1(x_var) }));
        let mut x_kurtosis: () = (((((6_()) * (unsafe { sqr_0(unimplemented!()) }))
            - ((6_()) * (unimplemented!())))
            + (1_()))
            / (x_var));
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
