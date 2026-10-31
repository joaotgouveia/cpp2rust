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
pub unsafe fn test_9() {
    let mut z: () = unimplemented!();
    assert!(((unimplemented!()) == (0_f32)));
}
pub unsafe fn test_10() {
    let mut z: () = unimplemented!();
    assert!(((unimplemented!()) == (0_f64)));
}
pub unsafe fn test_11() {
    let mut z: () = unimplemented!();
    assert!(((unimplemented!()) == (0_f64)));
}
pub unsafe fn test_edges_12() {
    let pi: f64 = unimplemented!();
    let N: u32 = (((::std::mem::size_of::<[(); 153]>() as usize)
        .wrapping_div((::std::mem::size_of::<()>() as usize))) as u32);
    let mut i: u32 = 0_u32;
    'loop_: while ((i) < (N)) {
        let mut r: f64 = unimplemented!();
        if (unimplemented!()) || (unimplemented!()) {
            assert!(unimplemented!());
        } else {
            'switch: {
                match {
                    (unsafe {
                        classify_4(
                            &(*std::cell::LazyCell::force_mut(&mut *&raw mut testcases_0))
                                [(i) as usize],
                        )
                    })
                } {
                    __v if __v == (anon_1_zero as i32) => {
                        if unimplemented!() {
                            if unimplemented!() {
                                (unsafe { is_about_7(r, -pi) });
                            } else {
                                (unsafe { is_about_7(r, pi) });
                            }
                        } else {
                            assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
                        }
                        break 'switch;
                    }
                    __v if __v == (anon_1_non_zero as i32) => {
                        if ((unimplemented!()) == (0_f64)) {
                            if ((unimplemented!()) < (0_f64)) {
                                (unsafe { is_about_7(r, ((-pi) / (2_f64))) });
                            } else {
                                (unsafe { is_about_7(r, ((pi) / (2_f64))) });
                            }
                        } else if ((unimplemented!()) == (0_f64)) {
                            if ((unimplemented!()) < (0_f64)) {
                                if unimplemented!() {
                                    (unsafe { is_about_7(r, -pi) });
                                } else {
                                    (unsafe { is_about_7(r, pi) });
                                }
                            } else {
                                assert!(((r) == (0_f64)));
                                assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
                            }
                        } else if ((unimplemented!()) > (0_f64)) {
                            assert!(((r) > (0_f64)));
                        } else {
                            assert!(((r) < (0_f64)));
                        }
                        break 'switch;
                    }
                    __v if __v == (anon_1_inf as i32) => {
                        if (unimplemented!()) && (unimplemented!()) {
                            if ((unimplemented!()) < (0_f64)) {
                                if ((unimplemented!()) > (0_f64)) {
                                    (unsafe { is_about_7(r, ((0.75_f64) * (pi))) });
                                } else {
                                    (unsafe { is_about_7(r, ((-0.75_f64) * (pi))) });
                                }
                            } else {
                                if ((unimplemented!()) > (0_f64)) {
                                    (unsafe { is_about_7(r, ((0.25_f64) * (pi))) });
                                } else {
                                    (unsafe { is_about_7(r, ((-0.25_f64) * (pi))) });
                                }
                            }
                        } else if unimplemented!() {
                            if ((unimplemented!()) < (0_f64)) {
                                if unimplemented!() {
                                    (unsafe { is_about_7(r, -pi) });
                                } else {
                                    (unsafe { is_about_7(r, pi) });
                                }
                            } else {
                                assert!(((r) == (0_f64)));
                                assert!(((unimplemented!() as i32) == (unimplemented!() as i32)));
                            }
                        } else {
                            if ((unimplemented!()) < (0_f64)) {
                                (unsafe { is_about_7(r, ((-pi) / (2_f64))) });
                            } else {
                                (unsafe { is_about_7(r, ((pi) / (2_f64))) });
                            }
                        }
                        break 'switch;
                    }
                    _ => {}
                }
            };
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
