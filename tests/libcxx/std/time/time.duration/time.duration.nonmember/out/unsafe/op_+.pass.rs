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
    {
        let mut s1: () = {
            let mut __tmp_0: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_0 })
        };
        let mut s2: () = {
            let mut __tmp_1: () = 5_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_1 })
        };
        let mut r: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = {
            let mut __tmp_2: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_2 })
        };
        let mut s2: () = {
            let mut __tmp_3: () = 5_();
            std_chrono_duration_long_long__std_ratio_1__1000000__::new_1({ &mut __tmp_3 })
        };
        let mut r: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = {
            let mut __tmp_4: () = 3_();
            std_chrono_duration_int__std_ratio_2__3__::new_1({ &mut __tmp_4 })
        };
        let mut s2: () = {
            let mut __tmp_5: () = 5_();
            std_chrono_duration_int__std_ratio_3__5__::new_1({ &mut __tmp_5 })
        };
        let mut r: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = {
            let mut __tmp_6: () = 3_();
            std_chrono_duration_int__std_ratio_2__3__::new_1({ &mut __tmp_6 })
        };
        let mut s2: () = {
            let mut __tmp_7: () = 5_();
            std_chrono_duration_double__std_ratio_3__5__::new_1({ &mut __tmp_7 })
        };
        let mut r: () = unimplemented!();
        unimplemented!();
    }
    {
        let s1: () = {
            let mut __tmp_8: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_8 })
        };
        let s2: () = {
            let mut __tmp_9: () = 5_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_9 })
        };
        let r: () = unimplemented!();
        const _: () = assert!(true, "r.count() == 8");;
    }
    {
        let s1: () = {
            let mut __tmp_10: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_10 })
        };
        let s2: () = {
            let mut __tmp_11: () = 5_();
            std_chrono_duration_long_long__std_ratio_1__1000000__::new_1({ &mut __tmp_11 })
        };
        let r: () = unimplemented!();
        const _: () = assert!(true, "r.count() == 3000005");;
    }
    {
        let s1: () = {
            let mut __tmp_12: () = 3_();
            std_chrono_duration_int__std_ratio_2__3__::new_1({ &mut __tmp_12 })
        };
        let s2: () = {
            let mut __tmp_13: () = 5_();
            std_chrono_duration_int__std_ratio_3__5__::new_1({ &mut __tmp_13 })
        };
        let r: () = unimplemented!();
        const _: () = assert!(true, "r.count() == 75");;
    }
    {
        let s1: () = {
            let mut __tmp_14: () = 3_();
            std_chrono_duration_int__std_ratio_2__3__::new_1({ &mut __tmp_14 })
        };
        let s2: () = {
            let mut __tmp_15: () = 5_();
            std_chrono_duration_double__std_ratio_3__5__::new_1({ &mut __tmp_15 })
        };
        let r: () = unimplemented!();
        const _: () = assert!(true, "r.count() == 75");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
