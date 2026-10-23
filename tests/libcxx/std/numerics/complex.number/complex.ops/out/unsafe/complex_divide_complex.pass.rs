extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_close_0(mut v: (), mut comp: ()) -> () {
    return ((v) <= ((comp) + (unsafe { std_numeric_limits_float_::epsilon() })))
        && ((v) >= ((comp) - (unsafe { std_numeric_limits_float_::epsilon() })));
}
pub unsafe fn is_close_1(mut v: (), mut comp: ()) -> () {
    return ((v) <= ((comp) + (unsafe { std_numeric_limits_double_::epsilon() })))
        && ((v) >= ((comp) - (unsafe { std_numeric_limits_double_::epsilon() })));
}
pub unsafe fn is_close_2(mut v: (), mut comp: ()) -> () {
    return ((v) <= ((comp) + (unsafe { std_numeric_limits_long_double_::epsilon() })))
        && ((v) >= ((comp) - (unsafe { std_numeric_limits_long_double_::epsilon() })));
}
pub static mut testcases_3: std::cell::LazyCell<[(); 153]> = std::cell::LazyCell::new(|| unsafe {
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
pub type anon_4 = ();
pub const anon_4_zero: anon_4 = 0;
pub const anon_4_non_zero: anon_4 = 1;
pub const anon_4_inf: anon_4 = 2;
pub const anon_4_NaN: anon_4 = 3;
pub const anon_4_non_zero_nan: anon_4 = 4;
pub unsafe fn test_isinf_5(mut v: ()) -> () {
    return ((v) == (unsafe { std_numeric_limits_double_::infinity() }))
        || ((v) == (-(unsafe { std_numeric_limits_double_::infinity() })));
}
pub unsafe fn test_isnan_6(mut v: ()) -> () {
    return ((v) != (v));
}
pub unsafe fn classify_7(x: *const ()) -> () {
    if unimplemented!() {
        return (anon_4_zero as ());
    }
    if (unsafe { test_isinf_5(unimplemented!()) }) || (unsafe { test_isinf_5(unimplemented!()) }) {
        return (anon_4_inf as ());
    }
    if (unsafe { test_isnan_6(unimplemented!()) }) && (unsafe { test_isnan_6(unimplemented!()) }) {
        return (anon_4_NaN as ());
    }
    if (unsafe { test_isnan_6(unimplemented!()) }) {
        if ((unimplemented!()) == (0_())) {
            return (anon_4_NaN as ());
        }
        return (anon_4_non_zero_nan as ());
    }
    if (unsafe { test_isnan_6(unimplemented!()) }) {
        if ((unimplemented!()) == (0_())) {
            return (anon_4_NaN as ());
        }
        return (anon_4_non_zero_nan as ());
    }
    return (anon_4_non_zero as ());
}
pub unsafe fn classify_8(mut x: ()) -> () {
    if ((x) == (0_())) {
        return (anon_4_zero as ());
    }
    if (unsafe { isinf_9(x) }) {
        return (anon_4_inf as ());
    }
    if (unsafe { isnan_10(x) }) {
        return (anon_4_NaN as ());
    }
    return (anon_4_non_zero as ());
}
pub unsafe fn is_about_11(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_12(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn is_about_13(mut x: (), mut y: ()) {
    unimplemented!();
}
pub unsafe fn test_14() -> () {
    let lhs: () = unimplemented!();
    let rhs: () = unimplemented!();
    let c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_15() -> () {
    let lhs: () = unimplemented!();
    let rhs: () = unimplemented!();
    let c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_16() -> () {
    let lhs: () = unimplemented!();
    let rhs: () = unimplemented!();
    let c: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_edges_17() -> () {
    let N: () = (::std::mem::size_of::<[(); 153]>() as ())
        .wrapping_div((::std::mem::size_of::<()>() as ()));
    let mut classification: [(); 153] = [unimplemented!(); 153];
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        classification[(i) as usize] = (unsafe {
            classify_7(&(*std::cell::LazyCell::force_mut(&mut *&raw mut testcases_3))[(i) as usize])
        });
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (N)) {
        let mut j: () = 0_();
        'loop_: while ((j) < (N)) {
            let mut r: () = unimplemented!();
            'switch: {
                match { classification[(i) as usize] } {
                    __v if __v == (anon_4_zero as ()) => {
                        'switch: {
                            match { classification[(j) as usize] } {
                                __v if __v == (anon_4_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_inf as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_NaN as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero_nan as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                _ => {}
                            }
                        };
                        break 'switch;
                    }
                    __v if __v == (anon_4_non_zero as ()) => {
                        'switch: {
                            match { classification[(j) as usize] } {
                                __v if __v == (anon_4_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_inf as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_NaN as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero_nan as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                _ => {}
                            }
                        };
                        break 'switch;
                    }
                    __v if __v == (anon_4_inf as ()) => {
                        'switch: {
                            match { classification[(j) as usize] } {
                                __v if __v == (anon_4_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_inf as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_NaN as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero_nan as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                _ => {}
                            }
                        };
                        break 'switch;
                    }
                    __v if __v == (anon_4_NaN as ()) => {
                        'switch: {
                            match { classification[(j) as usize] } {
                                __v if __v == (anon_4_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_inf as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_NaN as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero_nan as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                _ => {}
                            }
                        };
                        break 'switch;
                    }
                    __v if __v == (anon_4_non_zero_nan as ()) => {
                        'switch: {
                            match { classification[(j) as usize] } {
                                __v if __v == (anon_4_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_inf as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_NaN as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                __v if __v == (anon_4_non_zero_nan as ()) => {
                                    unimplemented!();
                                    break 'switch;
                                }
                                _ => {}
                            }
                        };
                        break 'switch;
                    }
                    _ => {}
                }
            };
            j.prefix_inc();
        }
        i.prefix_inc();
    }
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_14() });
    (unsafe { test_15() });
    (unsafe { test_16() });
    (unsafe { test_edges_17() });
    const _: () = assert!(true, "test<float>()");;
    const _: () = assert!(true, "test<double>()");;
    const _: () = assert!(true, "test<long double>()");;
    const _: () = assert!(true, "test_edges()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const testcases_3);
}
