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
            let mut __tmp_1: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_1 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = {
            let mut __tmp_2: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_2 })
        };
        let mut s2: () = {
            let mut __tmp_3: () = 4_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_3 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = {
            let mut __tmp_4: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_4 })
        };
        let mut s2: () = {
            let mut __tmp_5: () = 3000_();
            std_chrono_duration_long_long__std_ratio_1__1000000__::new_1({ &mut __tmp_5 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = {
            let mut __tmp_6: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_6 })
        };
        let mut s2: () = {
            let mut __tmp_7: () = 4000_();
            std_chrono_duration_long_long__std_ratio_1__1000000__::new_1({ &mut __tmp_7 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = {
            let mut __tmp_8: () = 9_();
            std_chrono_duration_int__std_ratio_2__3__::new_1({ &mut __tmp_8 })
        };
        let mut s2: () = {
            let mut __tmp_9: () = 10_();
            std_chrono_duration_int__std_ratio_3__5__::new_1({ &mut __tmp_9 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = {
            let mut __tmp_10: () = 10_();
            std_chrono_duration_int__std_ratio_2__3__::new_1({ &mut __tmp_10 })
        };
        let mut s2: () = {
            let mut __tmp_11: () = 9_();
            std_chrono_duration_int__std_ratio_3__5__::new_1({ &mut __tmp_11 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s1: () = {
            let mut __tmp_12: () = 9_();
            std_chrono_duration_int__std_ratio_2__3__::new_1({ &mut __tmp_12 })
        };
        let mut s2: () = {
            let mut __tmp_13: () = 10_();
            std_chrono_duration_double__std_ratio_3__5__::new_1({ &mut __tmp_13 })
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let s1: () = {
            let mut __tmp_14: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_14 })
        };
        let s2: () = {
            let mut __tmp_15: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_15 })
        };
        const _: () = assert!(true, "!(s1 < s2)");;
        const _: () = assert!(true, "!(s1 > s2)");;
        const _: () = assert!(true, "(s1 <= s2)");;
        const _: () = assert!(true, "(s1 >= s2)");;
    }
    {
        let s1: () = {
            let mut __tmp_16: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_16 })
        };
        let s2: () = {
            let mut __tmp_17: () = 4_();
            std_chrono_duration_long_long__std_ratio_1__1__::new_1({ &mut __tmp_17 })
        };
        const _: () = assert!(true, "(s1 < s2)");;
        const _: () = assert!(true, "!(s1 > s2)");;
        const _: () = assert!(true, "(s1 <= s2)");;
        const _: () = assert!(true, "!(s1 >= s2)");;
    }
    {
        let s1: () = {
            let mut __tmp_18: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_18 })
        };
        let s2: () = {
            let mut __tmp_19: () = 3000_();
            std_chrono_duration_long_long__std_ratio_1__1000000__::new_1({ &mut __tmp_19 })
        };
        const _: () = assert!(true, "!(s1 < s2)");;
        const _: () = assert!(true, "!(s1 > s2)");;
        const _: () = assert!(true, "(s1 <= s2)");;
        const _: () = assert!(true, "(s1 >= s2)");;
    }
    {
        let s1: () = {
            let mut __tmp_20: () = 3_();
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_20 })
        };
        let s2: () = {
            let mut __tmp_21: () = 4000_();
            std_chrono_duration_long_long__std_ratio_1__1000000__::new_1({ &mut __tmp_21 })
        };
        const _: () = assert!(true, "(s1 < s2)");;
        const _: () = assert!(true, "!(s1 > s2)");;
        const _: () = assert!(true, "(s1 <= s2)");;
        const _: () = assert!(true, "!(s1 >= s2)");;
    }
    {
        let s1: () = {
            let mut __tmp_22: () = 9_();
            std_chrono_duration_int__std_ratio_2__3__::new_1({ &mut __tmp_22 })
        };
        let s2: () = {
            let mut __tmp_23: () = 10_();
            std_chrono_duration_int__std_ratio_3__5__::new_1({ &mut __tmp_23 })
        };
        const _: () = assert!(true, "!(s1 < s2)");;
        const _: () = assert!(true, "!(s1 > s2)");;
        const _: () = assert!(true, "(s1 <= s2)");;
        const _: () = assert!(true, "(s1 >= s2)");;
    }
    {
        let s1: () = {
            let mut __tmp_24: () = 10_();
            std_chrono_duration_int__std_ratio_2__3__::new_1({ &mut __tmp_24 })
        };
        let s2: () = {
            let mut __tmp_25: () = 9_();
            std_chrono_duration_int__std_ratio_3__5__::new_1({ &mut __tmp_25 })
        };
        const _: () = assert!(true, "!(s1 < s2)");;
        const _: () = assert!(true, "(s1 > s2)");;
        const _: () = assert!(true, "!(s1 <= s2)");;
        const _: () = assert!(true, "(s1 >= s2)");;
    }
    {
        let s1: () = {
            let mut __tmp_26: () = 9_();
            std_chrono_duration_int__std_ratio_2__3__::new_1({ &mut __tmp_26 })
        };
        let s2: () = {
            let mut __tmp_27: () = 10_();
            std_chrono_duration_double__std_ratio_3__5__::new_1({ &mut __tmp_27 })
        };
        const _: () = assert!(true, "!(s1 < s2)");;
        const _: () = assert!(true, "!(s1 > s2)");;
        const _: () = assert!(true, "(s1 <= s2)");;
        const _: () = assert!(true, "(s1 >= s2)");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
