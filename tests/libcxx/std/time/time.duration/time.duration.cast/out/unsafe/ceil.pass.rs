extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_1(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, ToDuration>::value)");;
        unimplemented!();
    }
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
    (unsafe {
        let mut _f: () = {
            let mut __tmp_0: () = 7290000_();
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_0 })
        };
        let mut _d: () = {
            let mut __tmp_1: () = 3_();
            std_chrono_duration_long__std_ratio_3600__1__::new_1({ &mut __tmp_1 })
        };
        test_0(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = {
            let mut __tmp_2: () = -7290000_();
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_2 })
        };
        let mut _d: () = {
            let mut __tmp_3: () = -2_();
            std_chrono_duration_long__std_ratio_3600__1__::new_1({ &mut __tmp_3 })
        };
        test_0(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = {
            let mut __tmp_4: () = 7290000_();
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_4 })
        };
        let mut _d: () = {
            let mut __tmp_5: () = 122_();
            std_chrono_duration_long__std_ratio_60__1__::new_1({ &mut __tmp_5 })
        };
        test_1(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = {
            let mut __tmp_6: () = -7290000_();
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_6 })
        };
        let mut _d: () = {
            let mut __tmp_7: () = -121_();
            std_chrono_duration_long__std_ratio_60__1__::new_1({ &mut __tmp_7 })
        };
        test_1(&mut _f, &mut _d)
    });
    {
        let h1: () = (unsafe {
            let mut ___d: () = {
                let mut __tmp_8: () = 9000000_();
                std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_8 })
            };
            ceil_2(&mut ___d)
        });
        const _: () = assert!(true, "h1.count() == 3");;
        let h2: () = (unsafe {
            let mut ___d: () = {
                let mut __tmp_9: () = -9000000_();
                std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_9 })
            };
            ceil_2(&mut ___d)
        });
        const _: () = assert!(true, "h2.count() == -2");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
