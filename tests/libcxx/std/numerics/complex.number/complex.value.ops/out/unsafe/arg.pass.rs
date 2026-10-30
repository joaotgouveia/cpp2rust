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
    if unimplemented!() {
        return (anon_1_inf as ());
    }
    if unimplemented!() {
        return (anon_1_NaN as ());
    }
    return (anon_1_non_zero as ());
}
pub unsafe fn is_about_6(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_7(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_8(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn test_9() {
    let mut z: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_10() {
    let mut z: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_11() {
    let mut z: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_edges_12() {
    let pi: () = unimplemented!();
    let N: () = (::std::mem::size_of::<[(); 153]>() as ())
        .wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        let mut r: () = unimplemented!();
        if (unimplemented!()) || (unimplemented!()) {
            unimplemented!();
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
                    __v if __v == (anon_1_zero as ()) => {
                        if unimplemented!() {
                            if unimplemented!() {
                                (unsafe { is_about_7(r, -pi) });
                            } else {
                                (unsafe { is_about_7(r, pi) });
                            }
                        } else {
                            unimplemented!();
                        }
                        break 'switch;
                    }
                    __v if __v == (anon_1_non_zero as ()) => {
                        if ((unimplemented!()) == (0_())) {
                            if ((unimplemented!()) < (0_())) {
                                (unsafe { is_about_7(r, ((-pi) / (2_()))) });
                            } else {
                                (unsafe { is_about_7(r, ((pi) / (2_()))) });
                            }
                        } else if ((unimplemented!()) == (0_())) {
                            if ((unimplemented!()) < (0_())) {
                                if unimplemented!() {
                                    (unsafe { is_about_7(r, -pi) });
                                } else {
                                    (unsafe { is_about_7(r, pi) });
                                }
                            } else {
                                unimplemented!();
                                unimplemented!();
                            }
                        } else if ((unimplemented!()) > (0_())) {
                            unimplemented!();
                        } else {
                            unimplemented!();
                        }
                        break 'switch;
                    }
                    __v if __v == (anon_1_inf as ()) => {
                        if (unimplemented!()) && (unimplemented!()) {
                            if ((unimplemented!()) < (0_())) {
                                if ((unimplemented!()) > (0_())) {
                                    (unsafe { is_about_7(r, ((0.75_f64) * (pi))) });
                                } else {
                                    (unsafe { is_about_7(r, ((-0.75_f64) * (pi))) });
                                }
                            } else {
                                if ((unimplemented!()) > (0_())) {
                                    (unsafe { is_about_7(r, ((0.25_f64) * (pi))) });
                                } else {
                                    (unsafe { is_about_7(r, ((-0.25_f64) * (pi))) });
                                }
                            }
                        } else if unimplemented!() {
                            if ((unimplemented!()) < (0_())) {
                                if unimplemented!() {
                                    (unsafe { is_about_7(r, -pi) });
                                } else {
                                    (unsafe { is_about_7(r, pi) });
                                }
                            } else {
                                unimplemented!();
                                unimplemented!();
                            }
                        } else {
                            if ((unimplemented!()) < (0_())) {
                                (unsafe { is_about_7(r, ((-pi) / (2_()))) });
                            } else {
                                (unsafe { is_about_7(r, ((pi) / (2_()))) });
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
