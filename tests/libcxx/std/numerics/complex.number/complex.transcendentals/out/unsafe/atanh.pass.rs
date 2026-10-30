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
pub type anon_1 = ();
pub const anon_1_zero: anon_1 = 0;
pub const anon_1_non_zero: anon_1 = 1;
pub const anon_1_inf: anon_1 = 2;
pub const anon_1_NaN: anon_1 = 3;
pub const anon_1_non_zero_nan: anon_1 = 4;
pub unsafe fn classify_2(mut x: ()) -> () {
    if ((x) == (0_())) {
        return (anon_1_zero as ());
    }
    if unimplemented!() {
        return (anon_1_inf as ());
    }
    if unimplemented!() {
        return (anon_1_NaN as ());
    }
    return (anon_1_non_zero as ());
}
pub unsafe fn is_about_3(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_4(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_5(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn test_6(c: *const (), mut x: ()) {
    unimplemented!();
}
pub unsafe fn test_7(c: *const (), mut x: ()) {
    unimplemented!();
}
pub unsafe fn test_8(c: *const (), mut x: ()) {
    unimplemented!();
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
    let pi: () = unimplemented!();
    let N: () = (::std::mem::size_of::<[(); 153]>() as ())
        .wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        let mut r: () = unimplemented!();
        if ((unimplemented!()) == (0_())) && ((unimplemented!()) == (0_())) {
            unimplemented!();
            unimplemented!();
        } else if ((unimplemented!()) == (0_())) && (unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        } else if ((unimplemented!()) == (1_())) && ((unimplemented!()) == (0_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        } else if (unimplemented!()) && (unimplemented!()) {
            unimplemented!();
            unimplemented!();
            if ((unimplemented!()) > (0_())) {
                (unsafe { is_about_4(unimplemented!(), ((pi) / (2_()))) });
            } else {
                (unsafe { is_about_4(unimplemented!(), ((-pi) / (2_()))) });
            }
        } else if (unimplemented!()) && (unimplemented!()) {
            unimplemented!();
            unimplemented!();
        } else if (unimplemented!()) && (unimplemented!()) {
            unimplemented!();
            unimplemented!();
            if unimplemented!() {
                (unsafe { is_about_4(unimplemented!(), ((-pi) / (2_()))) });
            } else {
                (unsafe { is_about_4(unimplemented!(), ((pi) / (2_()))) });
            }
        } else if (unimplemented!()) && (unimplemented!()) {
            unimplemented!();
            unimplemented!();
            if unimplemented!() {
                (unsafe { is_about_4(unimplemented!(), ((-pi) / (2_()))) });
            } else {
                (unsafe { is_about_4(unimplemented!(), ((pi) / (2_()))) });
            }
        } else if (unimplemented!()) && (unimplemented!()) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
        } else if (unimplemented!()) && (unimplemented!()) {
            unimplemented!();
            unimplemented!();
        } else if (unimplemented!()) && (unimplemented!()) {
            unimplemented!();
            unimplemented!();
            if unimplemented!() {
                (unsafe { is_about_4(unimplemented!(), ((-pi) / (2_()))) });
            } else {
                (unsafe { is_about_4(unimplemented!(), ((pi) / (2_()))) });
            }
        } else if (unimplemented!()) && (unimplemented!()) {
            unimplemented!();
            unimplemented!();
        } else {
            unimplemented!();
            unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_9() });
    (unsafe { test_10() });
    (unsafe { test_11() });
    (unsafe { test_edges_12() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const testcases_0);
}
