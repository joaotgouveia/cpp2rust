extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut a1_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut a1_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut a2_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut a2_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn testOne_4() {
    const _: () = assert!(true, "std::is_nothrow_constructible_v<std::atomic<T>>");;
    const _: () = assert!(true, "std::is_nothrow_constructible_v<std::atomic<T>, T>");;
    {
        let mut a: () = unimplemented!();
        {
            assert!(((unimplemented!()) == (0_f32)));
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            assert!(((unimplemented!()) == (5.2_f64 as f32)));
        }
    }
    {
        assert!(((unimplemented!()) == (0_f64 as f32)));
        assert!(((unimplemented!()) == (5.2_f64 as f32)));
    }
}
pub unsafe fn testOne_5() {
    const _: () = assert!(true, "std::is_nothrow_constructible_v<std::atomic<T>>");;
    const _: () = assert!(true, "std::is_nothrow_constructible_v<std::atomic<T>, T>");;
    {
        let mut a: () = unimplemented!();
        {
            assert!(((unimplemented!()) == (0_f64)));
        }
    }
    {
        let mut a: () = unimplemented!();
        {
            assert!(((unimplemented!()) == (5.2_f64)));
        }
    }
    {
        assert!(((unimplemented!()) == (0_f64)));
        assert!(((unimplemented!()) == (5.2_f64)));
    }
}
pub unsafe fn test_6() -> bool {
    (unsafe { testOne_4() });
    (unsafe { testOne_5() });
    return true;
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
    (unsafe { test_6() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const a1_0);
    std::cell::LazyCell::force(&*&raw const a1_1);
    std::cell::LazyCell::force(&*&raw const a2_2);
    std::cell::LazyCell::force(&*&raw const a2_3);
}
