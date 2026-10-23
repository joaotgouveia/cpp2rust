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
pub unsafe fn test_isinf_2(mut v: ()) -> () {
    return ((v) == (unimplemented!())) || ((v) == (-unimplemented!()));
}
pub unsafe fn test_isnan_3(mut v: ()) -> () {
    return ((v) != (v));
}
pub unsafe fn classify_4(x: *const ()) -> () {
    if unimplemented!() {
        return (anon_1_zero as ());
    }
    if (unsafe { test_isinf_2(unimplemented!()) }) || (unsafe { test_isinf_2(unimplemented!()) }) {
        return (anon_1_inf as ());
    }
    if (unsafe { test_isnan_3(unimplemented!()) }) && (unsafe { test_isnan_3(unimplemented!()) }) {
        return (anon_1_NaN as ());
    }
    if (unsafe { test_isnan_3(unimplemented!()) }) {
        if ((unimplemented!()) == (0_())) {
            return (anon_1_NaN as ());
        }
        return (anon_1_non_zero_nan as ());
    }
    if (unsafe { test_isnan_3(unimplemented!()) }) {
        if ((unimplemented!()) == (0_())) {
            return (anon_1_NaN as ());
        }
        return (anon_1_non_zero_nan as ());
    }
    return (anon_1_non_zero as ());
}
pub unsafe fn classify_5(mut x: ()) -> () {
    if ((x) == (0_())) {
        return (anon_1_zero as ());
    }
    if (unsafe { isinf_6(x) }) {
        return (anon_1_inf as ());
    }
    if (unsafe { isnan_7(x) }) {
        return (anon_1_NaN as ());
    }
    return (anon_1_non_zero as ());
}
pub unsafe fn is_about_8(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_9(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_10(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn test_11(rho: *const (), mut x: ()) {
    unimplemented!();
}
pub unsafe fn test_12(rho: *const (), mut x: ()) {
    unimplemented!();
}
pub unsafe fn test_13(rho: *const (), mut x: ()) {
    unimplemented!();
}
pub unsafe fn test_14(rho: *const (), theta: *const (), mut x: ()) {
    unimplemented!();
}
pub unsafe fn test_15(rho: *const (), theta: *const (), mut x: ()) {
    unimplemented!();
}
pub unsafe fn test_16(rho: *const (), theta: *const (), mut x: ()) {
    unimplemented!();
}
pub unsafe fn test_17() {
    (unsafe {
        let mut _rho: () = 0_();
        test_11(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 1_();
        test_11(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 100_();
        test_11(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 0_();
        let mut _theta: () = 0_();
        test_14(&mut _rho, &mut _theta, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 1_();
        let mut _theta: () = 0_();
        test_14(&mut _rho, &mut _theta, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 100_();
        let mut _theta: () = 0_();
        test_14(&mut _rho, &mut _theta, unimplemented!())
    });
}
pub unsafe fn test_18() {
    (unsafe {
        let mut _rho: () = 0_();
        test_12(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 1_();
        test_12(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 100_();
        test_12(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 0_();
        let mut _theta: () = 0_();
        test_15(&mut _rho, &mut _theta, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 1_();
        let mut _theta: () = 0_();
        test_15(&mut _rho, &mut _theta, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 100_();
        let mut _theta: () = 0_();
        test_15(&mut _rho, &mut _theta, unimplemented!())
    });
}
pub unsafe fn test_19() {
    (unsafe {
        let mut _rho: () = 0_();
        test_13(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 1_();
        test_13(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 100_();
        test_13(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 0_();
        let mut _theta: () = 0_();
        test_16(&mut _rho, &mut _theta, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 1_();
        let mut _theta: () = 0_();
        test_16(&mut _rho, &mut _theta, unimplemented!())
    });
    (unsafe {
        let mut _rho: () = 100_();
        let mut _theta: () = 0_();
        test_16(&mut _rho, &mut _theta, unimplemented!())
    });
}
pub unsafe fn test_edges_20() {
    let N: () = (::std::mem::size_of::<[(); 153]>() as ())
        .wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        let mut r: () = unimplemented!();
        let mut theta: () = unimplemented!();
        let mut z: () = unimplemented!();
        'switch: {
            match { (unsafe { classify_5(r) }) } {
                __v if __v == (anon_1_zero as ()) => {
                    if ((unsafe { signbit_21(r) })
                        || ((unsafe { classify_5(theta) }) == (anon_1_inf as ())))
                        || ((unsafe { classify_5(theta) }) == (anon_1_NaN as ()))
                    {
                        let mut c: () = (unsafe { classify_4(&z) });
                        unimplemented!();
                    } else {
                        unimplemented!();
                    }
                    break 'switch;
                }
                __v if __v == (anon_1_non_zero as ()) => {
                    if ((unsafe { signbit_21(r) })
                        || ((unsafe { classify_5(theta) }) == (anon_1_inf as ())))
                        || ((unsafe { classify_5(theta) }) == (anon_1_NaN as ()))
                    {
                        let mut c: () = (unsafe { classify_4(&z) });
                        unimplemented!();
                    } else {
                        (unsafe { is_about_9(unimplemented!(), r) });
                    }
                    break 'switch;
                }
                __v if __v == (anon_1_inf as ()) => {
                    if ((r) < (0_())) {
                        let mut c: () = (unsafe { classify_4(&z) });
                        unimplemented!();
                    } else {
                        unimplemented!();
                        if ((unsafe { classify_5(theta) }) != (anon_1_NaN as ()))
                            && ((unsafe { classify_5(theta) }) != (anon_1_inf as ()))
                        {
                            unimplemented!();
                            unimplemented!();
                        }
                    }
                    break 'switch;
                }
                __v if __v == (anon_1_NaN as ()) || __v == (anon_1_non_zero_nan as ()) => {
                    {
                        let mut c: () = (unsafe { classify_4(&z) });
                        unimplemented!();
                    }
                    break 'switch;
                }
                _ => {}
            }
        };
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
    (unsafe { test_17() });
    (unsafe { test_18() });
    (unsafe { test_19() });
    (unsafe { test_edges_20() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const testcases_0);
}
