extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut testcases_0: std::cell::LazyCell<[(); 153]> = std::cell::LazyCell::new(|| unsafe {
    [
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
        unimplemented!(),
    ]
});
pub type anon_1 = u32;
pub const anon_1_zero: anon_1 = 0;
pub const anon_1_non_zero: anon_1 = 1;
pub const anon_1_inf: anon_1 = 2;
pub const anon_1_NaN: anon_1 = 3;
pub const anon_1_non_zero_nan: anon_1 = 4;
pub unsafe fn classify_2(mut x: f64) -> i32 {
    if ((x) == (0_f64)) {
        return (anon_1_zero as i32);
    }
    if unimplemented!() {
        return (anon_1_inf as i32);
    }
    if unimplemented!() {
        return (anon_1_NaN as i32);
    }
    return (anon_1_non_zero as i32);
}
pub unsafe fn is_about_3(mut x: f32, mut y: f32) {
    assert!(((unimplemented!() as f64) < (1e-06_f64)));
}
pub unsafe fn is_about_4(mut x: f64, mut y: f64) {
    assert!(((unimplemented!()) < (1e-14_f64)));
}
pub unsafe fn is_about_5(mut x: f64, mut y: f64) {
    assert!(((unimplemented!()) < (1e-14_f64)));
}
pub unsafe fn test_6(c: *const (), mut x: ()) {
    let mut a: () = unimplemented!();
    (unsafe {
        let _x: f32 = unimplemented!();
        let _y: f32 = unimplemented!();
        is_about_3(_x, _y)
    });
    assert!(((unimplemented!() as f64) < (1e-06_f64)));
}
pub unsafe fn test_7(c: *const (), mut x: ()) {
    let mut a: () = unimplemented!();
    (unsafe {
        let _x: f64 = unimplemented!();
        let _y: f64 = unimplemented!();
        is_about_4(_x, _y)
    });
    assert!(((unimplemented!()) < (1e-06_f64)));
}
pub unsafe fn test_8(c: *const (), mut x: ()) {
    let mut a: () = unimplemented!();
    (unsafe {
        let _x: f64 = unimplemented!();
        let _y: f64 = unimplemented!();
        is_about_5(_x, _y)
    });
    assert!(((unimplemented!()) < (1e-06_f64)));
}
pub unsafe fn test_9() {
    (unsafe {
        let mut _c: () = unimplemented!();
        test_6(&mut _c, unimplemented!())
    });
}
pub unsafe fn test_10() {
    (unsafe {
        let mut _c: () = unimplemented!();
        test_7(&mut _c, unimplemented!())
    });
}
pub unsafe fn test_11() {
    (unsafe {
        let mut _c: () = unimplemented!();
        test_8(&mut _c, unimplemented!())
    });
}
pub unsafe fn test_edges_12() {
    let N: u32 = (((::std::mem::size_of::<[(); 153]>() as usize)
        .wrapping_div((::std::mem::size_of::<()>() as usize))) as u32);
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (N)) {
        let mut r: () = unimplemented!();
        if ((unimplemented!()) == (0_f64)) && ((unimplemented!()) == (0_f64)) {
            assert!(!(unimplemented!()));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
        } else if unimplemented!() {
            assert!(unimplemented!());
            assert!(((unimplemented!()) > (0_f64)));
            assert!(unimplemented!());
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
        } else if (unimplemented!()) && (unimplemented!()) {
            assert!(unimplemented!());
            assert!(unimplemented!());
        } else if ((unimplemented!()) && ((unimplemented!()) < (0_f64))) && (unimplemented!()) {
            assert!(((unimplemented!()) == (0_f64)));
            assert!(!(unimplemented!()));
            assert!(unimplemented!());
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
        } else if ((unimplemented!()) && ((unimplemented!()) > (0_f64))) && (unimplemented!()) {
            assert!(unimplemented!());
            assert!(((unimplemented!()) > (0_f64)));
            assert!(((unimplemented!()) == (0_f64)));
            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
        } else if ((unimplemented!()) && ((unimplemented!()) < (0_f64))) && (unimplemented!()) {
            assert!(unimplemented!());
            assert!(unimplemented!());
        } else if ((unimplemented!()) && ((unimplemented!()) > (0_f64))) && (unimplemented!()) {
            assert!(unimplemented!());
            assert!(((unimplemented!()) > (0_f64)));
            assert!(unimplemented!());
        } else if (unimplemented!()) && ((unimplemented!()) || (unimplemented!())) {
            assert!(unimplemented!());
            assert!(unimplemented!());
        } else if unimplemented!() {
            assert!(!(unimplemented!()));
            assert!(unimplemented!());
        } else {
            assert!(!(unimplemented!()));
            assert!(!(unimplemented!()));
        }
        i.prefix_inc();
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_9() });
    (unsafe { test_10() });
    (unsafe { test_11() });
    (unsafe { test_edges_12() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const testcases_0);
}
