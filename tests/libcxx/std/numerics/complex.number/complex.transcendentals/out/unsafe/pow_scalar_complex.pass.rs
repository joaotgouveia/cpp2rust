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
    if (unsafe { isinf_3(x) }) {
        return (anon_1_inf as ());
    }
    if (unsafe { isnan_4(x) }) {
        return (anon_1_NaN as ());
    }
    return (anon_1_non_zero as ());
}
pub unsafe fn is_about_5(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_6(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_7(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn test_8(a: *const (), b: *const (), mut x: ()) {
    let mut c: () = (unsafe {
        let ___x: *const () = a;
        let ___y: *const () = b;
        pow_9(___x, ___y)
    });
    (unsafe {
        let _x: () = unimplemented!();
        let _y: () = unimplemented!();
        is_about_5(_x, _y)
    });
    unimplemented!();
}
pub unsafe fn test_10(a: *const (), b: *const (), mut x: ()) {
    let mut c: () = (unsafe {
        let ___x: *const () = a;
        let ___y: *const () = b;
        pow_11(___x, ___y)
    });
    (unsafe {
        let _x: () = unimplemented!();
        let _y: () = unimplemented!();
        is_about_6(_x, _y)
    });
    unimplemented!();
}
pub unsafe fn test_12(a: *const (), b: *const (), mut x: ()) {
    let mut c: () = (unsafe {
        let ___x: *const () = a;
        let ___y: *const () = b;
        pow_13(___x, ___y)
    });
    (unsafe {
        let _x: () = unimplemented!();
        let _y: () = unimplemented!();
        is_about_7(_x, _y)
    });
    unimplemented!();
}
pub unsafe fn test_14() {
    (unsafe {
        let mut _a: () = 2_();
        let mut _b: () = unimplemented!();
        test_8(&mut _a, &mut _b, unimplemented!())
    });
}
pub unsafe fn test_15() {
    (unsafe {
        let mut _a: () = 2_();
        let mut _b: () = unimplemented!();
        test_10(&mut _a, &mut _b, unimplemented!())
    });
}
pub unsafe fn test_16() {
    (unsafe {
        let mut _a: () = 2_();
        let mut _b: () = unimplemented!();
        test_12(&mut _a, &mut _b, unimplemented!())
    });
}
pub unsafe fn test_edges_17() {
    let N: () = (::std::mem::size_of::<[(); 153]>() as ())
        .wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        let mut j: () = 0_();
        'loop_: while ((j) < (N)) {
            let mut r: () = (unsafe {
                let mut ___x: () = unimplemented!();
                let ___y: *const () =
                    &(*std::cell::LazyCell::force_mut(&mut *&raw mut testcases_0))[(j) as usize];
                pow_11(&mut ___x, ___y)
            });
            let mut z: () = unimplemented!();
            if (unsafe { isnan_4(unimplemented!()) }) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            if (unsafe { isnan_4(unimplemented!()) }) {
                unimplemented!();
            } else {
                unimplemented!();
            }
            j.prefix_inc();
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
    (unsafe { test_14() });
    (unsafe { test_15() });
    (unsafe { test_16() });
    (unsafe { test_edges_17() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const testcases_0);
}
