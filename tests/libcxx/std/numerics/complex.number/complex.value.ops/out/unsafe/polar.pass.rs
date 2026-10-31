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
pub unsafe fn test_isinf_2(mut v: f64) -> bool {
    return ((v) == (unimplemented!())) || ((v) == (-unimplemented!()));
}
pub unsafe fn test_isnan_3(mut v: f64) -> bool {
    return ((v) != (v));
}
pub unsafe fn classify_4(x: *const ()) -> i32 {
    if unimplemented!() {
        return (anon_1_zero as i32);
    }
    if (unsafe { test_isinf_2(unimplemented!()) }) || (unsafe { test_isinf_2(unimplemented!()) }) {
        return (anon_1_inf as i32);
    }
    if (unsafe { test_isnan_3(unimplemented!()) }) && (unsafe { test_isnan_3(unimplemented!()) }) {
        return (anon_1_NaN as i32);
    }
    if (unsafe { test_isnan_3(unimplemented!()) }) {
        if ((unimplemented!()) == (0_f64)) {
            return (anon_1_NaN as i32);
        }
        return (anon_1_non_zero_nan as i32);
    }
    if (unsafe { test_isnan_3(unimplemented!()) }) {
        if ((unimplemented!()) == (0_f64)) {
            return (anon_1_NaN as i32);
        }
        return (anon_1_non_zero_nan as i32);
    }
    return (anon_1_non_zero as i32);
}
pub unsafe fn classify_5(mut x: f64) -> i32 {
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
pub unsafe fn is_about_6(mut x: f32, mut y: f32) {
    assert!(((unimplemented!() as f64) < (1e-06_f64)));
}
pub unsafe fn is_about_7(mut x: f64, mut y: f64) {
    assert!(((unimplemented!()) < (1e-14_f64)));
}
pub unsafe fn is_about_8(mut x: f64, mut y: f64) {
    assert!(((unimplemented!()) < (1e-14_f64)));
}
pub unsafe fn test_9(rho: *const f32, mut x: ()) {
    assert!(unimplemented!());
}
pub unsafe fn test_10(rho: *const f64, mut x: ()) {
    assert!(unimplemented!());
}
pub unsafe fn test_11(rho: *const f64, mut x: ()) {
    assert!(unimplemented!());
}
pub unsafe fn test_12(rho: *const f32, theta: *const f32, mut x: ()) {
    assert!(unimplemented!());
}
pub unsafe fn test_13(rho: *const f64, theta: *const f64, mut x: ()) {
    assert!(unimplemented!());
}
pub unsafe fn test_14(rho: *const f64, theta: *const f64, mut x: ()) {
    assert!(unimplemented!());
}
pub unsafe fn test_15() {
    (unsafe {
        let mut _rho: f32 = 0_f32;
        test_9(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: f32 = 1_f32;
        test_9(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: f32 = 100_f32;
        test_9(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: f32 = 0_f32;
        let mut _theta: f32 = 0_f32;
        test_12(&mut _rho, &mut _theta, unimplemented!())
    });
    (unsafe {
        let mut _rho: f32 = 1_f32;
        let mut _theta: f32 = 0_f32;
        test_12(&mut _rho, &mut _theta, unimplemented!())
    });
    (unsafe {
        let mut _rho: f32 = 100_f32;
        let mut _theta: f32 = 0_f32;
        test_12(&mut _rho, &mut _theta, unimplemented!())
    });
}
pub unsafe fn test_16() {
    (unsafe {
        let mut _rho: f64 = 0_f64;
        test_10(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: f64 = 1_f64;
        test_10(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: f64 = 100_f64;
        test_10(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: f64 = 0_f64;
        let mut _theta: f64 = 0_f64;
        test_13(&mut _rho, &mut _theta, unimplemented!())
    });
    (unsafe {
        let mut _rho: f64 = 1_f64;
        let mut _theta: f64 = 0_f64;
        test_13(&mut _rho, &mut _theta, unimplemented!())
    });
    (unsafe {
        let mut _rho: f64 = 100_f64;
        let mut _theta: f64 = 0_f64;
        test_13(&mut _rho, &mut _theta, unimplemented!())
    });
}
pub unsafe fn test_17() {
    (unsafe {
        let mut _rho: f64 = 0_f64;
        test_11(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: f64 = 1_f64;
        test_11(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: f64 = 100_f64;
        test_11(&mut _rho, unimplemented!())
    });
    (unsafe {
        let mut _rho: f64 = 0_f64;
        let mut _theta: f64 = 0_f64;
        test_14(&mut _rho, &mut _theta, unimplemented!())
    });
    (unsafe {
        let mut _rho: f64 = 1_f64;
        let mut _theta: f64 = 0_f64;
        test_14(&mut _rho, &mut _theta, unimplemented!())
    });
    (unsafe {
        let mut _rho: f64 = 100_f64;
        let mut _theta: f64 = 0_f64;
        test_14(&mut _rho, &mut _theta, unimplemented!())
    });
}
pub unsafe fn test_edges_18() {
    let N: u32 = (((::std::mem::size_of::<[(); 153]>() as usize)
        .wrapping_div((::std::mem::size_of::<()>() as usize))) as u32);
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (N)) {
        let mut r: f64 = unimplemented!();
        let mut theta: f64 = unimplemented!();
        let mut z: () = unimplemented!();
        'switch: {
            match { (unsafe { classify_5(r) }) } {
                __v if __v == (anon_1_zero as i32) => {
                    if ((unimplemented!())
                        || ((unsafe { classify_5(theta) }) == (anon_1_inf as i32)))
                        || ((unsafe { classify_5(theta) }) == (anon_1_NaN as i32))
                    {
                        let mut c: i32 = (unsafe { classify_4(&z) });
                        assert!(
                            ((c) == (anon_1_NaN as i32)) || ((c) == (anon_1_non_zero_nan as i32))
                        );
                    } else {
                        assert!(unimplemented!());
                    }
                    break 'switch;
                }
                __v if __v == (anon_1_non_zero as i32) => {
                    if ((unimplemented!())
                        || ((unsafe { classify_5(theta) }) == (anon_1_inf as i32)))
                        || ((unsafe { classify_5(theta) }) == (anon_1_NaN as i32))
                    {
                        let mut c: i32 = (unsafe { classify_4(&z) });
                        assert!(
                            ((c) == (anon_1_NaN as i32)) || ((c) == (anon_1_non_zero_nan as i32))
                        );
                    } else {
                        (unsafe { is_about_7(unimplemented!(), r) });
                    }
                    break 'switch;
                }
                __v if __v == (anon_1_inf as i32) => {
                    if ((r) < (0_f64)) {
                        let mut c: i32 = (unsafe { classify_4(&z) });
                        assert!(
                            ((c) == (anon_1_NaN as i32)) || ((c) == (anon_1_non_zero_nan as i32))
                        );
                    } else {
                        assert!(((unsafe { classify_4(&z,) }) == (anon_1_inf as i32)));
                        if ((unsafe { classify_5(theta) }) != (anon_1_NaN as i32))
                            && ((unsafe { classify_5(theta) }) != (anon_1_inf as i32))
                        {
                            assert!(
                                ((unsafe { classify_5(unimplemented!(),) }) != (anon_1_NaN as i32))
                            );
                            assert!(
                                ((unsafe { classify_5(unimplemented!(),) }) != (anon_1_NaN as i32))
                            );
                        }
                    }
                    break 'switch;
                }
                __v if __v == (anon_1_NaN as i32) || __v == (anon_1_non_zero_nan as i32) => {
                    {
                        let mut c: i32 = (unsafe { classify_4(&z) });
                        assert!(
                            ((c) == (anon_1_NaN as i32)) || ((c) == (anon_1_non_zero_nan as i32))
                        );
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_15() });
    (unsafe { test_16() });
    (unsafe { test_17() });
    (unsafe { test_edges_18() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const testcases_0);
}
