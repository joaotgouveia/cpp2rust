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
    {
        let c: Vec<Option<Box<f64>>> = Vec::from([Some(Box::from_raw(
            (Box::leak(Box::new(3.5_f64)) as *mut f64),
        ))]);
        const _: () = assert!(
            true,
            "std::is_same<const T&&, decltype(std::get<0>(std::move(c)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<0>(std::move(c)))");;
        let t: *const Option<Box<f64>> = unimplemented!();
        assert!(
            ((*(*t)
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const f64 as *mut f64))
                == (3.5_f64))
        );
    }
    {
        let c: Vec<f64> = Vec::from([1_f64, 2_f64, 3.5_f64]);
        const _: () = assert!(true, "std::get<0>(std::move(c)) == 1");;
        const _: () = assert!(true, "std::get<1>(std::move(c)) == 2");;
        const _: () = assert!(true, "std::get<2>(std::move(c)) == 3.5");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
