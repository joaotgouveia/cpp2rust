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
    const _: () = assert!(
        true,
        "std::is_same<decltype( 3.0il ), std::complex<long double>>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype( 3il   ), std::complex<long double>>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype( 3.0i  ), std::complex<double>>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype( 3i    ), std::complex<double>>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype( 3.0if ), std::complex<float>>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype( 3if   ), std::complex<float>>::value"
    );;
    {
        let mut c1: () = (unsafe { operator_literal_il_0(3_f64) });
        unimplemented!();
        let mut c2: () = (unsafe { operator_literal_il_1(3_()) });
        unimplemented!();
    }
    {
        let mut c1: () = (unsafe { operator_literal_i_2(3_f64) });
        unimplemented!();
        let mut c2: () = (unsafe { operator_literal_i_3(3_()) });
        unimplemented!();
    }
    {
        let mut c1: () = (unsafe { operator_literal_if_4(3_f64) });
        unimplemented!();
        let mut c2: () = (unsafe { operator_literal_if_5(3_()) });
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
