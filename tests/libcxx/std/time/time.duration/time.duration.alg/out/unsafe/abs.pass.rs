extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut has_abs_v_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { true });
pub static mut has_abs_v_1: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { false });
const _: () = assert!(true, "has_abs_v<std::chrono::milliseconds>");
const _: () = assert!(true, "!has_abs_v<std::chrono::duration<unsigned>>");
pub unsafe fn test_2(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, Duration>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_3(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, Duration>::value)");;
        unimplemented!();
    }
}
pub unsafe fn test_4(f: *const (), d: *const ()) {
    {
        const _: () = assert!(true, "(std::is_same<R, Duration>::value)");;
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
            let mut __tmp_1: () = 7290000_();
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_1 })
        };
        test_2(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = {
            let mut __tmp_2: () = -7290000_();
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_2 })
        };
        let mut _d: () = {
            let mut __tmp_3: () = 7290000_();
            std_chrono_duration_long_long__std_ratio_1__1000__::new_1({ &mut __tmp_3 })
        };
        test_2(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = {
            let mut __tmp_4: () = 122_();
            std_chrono_duration_long__std_ratio_60__1__::new_1({ &mut __tmp_4 })
        };
        let mut _d: () = {
            let mut __tmp_5: () = 122_();
            std_chrono_duration_long__std_ratio_60__1__::new_1({ &mut __tmp_5 })
        };
        test_3(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = {
            let mut __tmp_6: () = -122_();
            std_chrono_duration_long__std_ratio_60__1__::new_1({ &mut __tmp_6 })
        };
        let mut _d: () = {
            let mut __tmp_7: () = 122_();
            std_chrono_duration_long__std_ratio_60__1__::new_1({ &mut __tmp_7 })
        };
        test_3(&mut _f, &mut _d)
    });
    (unsafe {
        let mut _f: () = {
            let mut __tmp_8: () = 0_();
            std_chrono_duration_long__std_ratio_3600__1__::new_1({ &mut __tmp_8 })
        };
        let mut _d: () = {
            let mut __tmp_9: () = 0_();
            std_chrono_duration_long__std_ratio_3600__1__::new_1({ &mut __tmp_9 })
        };
        test_4(&mut _f, &mut _d)
    });
    {
        let h1: () = (unsafe {
            abs_5({
                let mut __tmp_10: () = -3_();
                std_chrono_duration_long__std_ratio_3600__1__::new_1({ &mut __tmp_10 })
            })
        });
        const _: () = assert!(true, "h1.count() == 3");;
        let h2: () = (unsafe {
            abs_5({
                let mut __tmp_11: () = 3_();
                std_chrono_duration_long__std_ratio_3600__1__::new_1({ &mut __tmp_11 })
            })
        });
        const _: () = assert!(true, "h2.count() == 3");;
    }
    {
        let d: () = (unsafe {
            abs_6({
                let mut __tmp_12: () = 2_();
                std_chrono_duration_int__std_ratio_60__100__::new_1({ &mut __tmp_12 })
            })
        });
        const _: () = assert!(true, "d.count() == 2");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const has_abs_v_0);
    std::cell::LazyCell::force(&*&raw const has_abs_v_1);
}
