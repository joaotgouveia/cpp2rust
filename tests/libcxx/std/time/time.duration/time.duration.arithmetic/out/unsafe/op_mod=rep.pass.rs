extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NotARep {}
pub unsafe fn operator_rem_assign_0(mut d: (), mut _a1: NotARep) -> () {
    return d;
}
pub unsafe fn test_constexpr_1() -> () {
    let mut s: () = {
        let mut __tmp_0: () = 11_();
        std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_0 })
    };
    unimplemented!();
    return ((unimplemented!()) == (2_()));
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
        let mut us: () = {
            let mut __tmp_1: () = 11_();
            std_chrono_duration_long_long__std_ratio_1__1000000__::new_1({ &mut __tmp_1 })
        };
        unimplemented!();
        unimplemented!();
    }
    const _: () = assert!(true, "test_constexpr()");;
    {
        let mut d: () = {
            let mut __tmp_2: () = 5_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_2 })
        };
        let mut n: NotARep = <NotARep>::default();
        (unsafe {
            let _d: () = d.clone();
            operator_rem_assign_0(_d, n)
        });
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
