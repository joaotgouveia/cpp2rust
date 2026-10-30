extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_float__float_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_double__double_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_long_double__long_double_ {}
pub unsafe fn fptest_close_3(mut val: (), mut expected: (), mut eps: ()) -> () {
    let zero: () = 0_();
    unimplemented!();
    if ((eps) == (zero)) {
        return ((val) == (expected));
    }
    if ((val) == (zero)) {
        return ((unimplemented!()) <= (eps));
    }
    if ((expected) == (zero)) {
        return ((unimplemented!()) <= (eps));
    }
    return ((unimplemented!()) < (eps)) && (((unimplemented!()) / (unimplemented!())) < (eps));
}
pub unsafe fn fptest_close_4(mut val: (), mut expected: (), mut eps: ()) -> () {
    let zero: () = 0_();
    unimplemented!();
    if ((eps) == (zero)) {
        return ((val) == (expected));
    }
    if ((val) == (zero)) {
        return ((unimplemented!()) <= (eps));
    }
    if ((expected) == (zero)) {
        return ((unimplemented!()) <= (eps));
    }
    return ((unimplemented!()) < (eps)) && (((unimplemented!()) / (unimplemented!())) < (eps));
}
pub unsafe fn fptest_close_5(mut val: (), mut expected: (), mut eps: ()) -> () {
    let zero: () = 0_();
    unimplemented!();
    if ((eps) == (zero)) {
        return ((val) == (expected));
    }
    if ((val) == (zero)) {
        return ((unimplemented!()) <= (eps));
    }
    if ((expected) == (zero)) {
        return ((unimplemented!()) <= (eps));
    }
    return ((unimplemented!()) < (eps)) && (((unimplemented!()) / (unimplemented!())) < (eps));
}
pub unsafe fn fptest_close_pct_6(mut val: (), mut expected: (), mut percent: ()) -> () {
    unimplemented!();
    let mut eps: () = (((percent) / (100_())) * (*unimplemented!()));
    return (unsafe { fptest_close_3(val, expected, eps) });
}
pub unsafe fn fptest_close_pct_7(mut val: (), mut expected: (), mut percent: ()) -> () {
    unimplemented!();
    let mut eps: () = (((percent) / (100_())) * (*unimplemented!()));
    return (unsafe { fptest_close_4(val, expected, eps) });
}
pub unsafe fn fptest_close_pct_8(mut val: (), mut expected: (), mut percent: ()) -> () {
    unimplemented!();
    let mut eps: () = (((percent) / (100_())) * (*unimplemented!()));
    return (unsafe { fptest_close_5(val, expected, eps) });
}
pub unsafe fn is_nan_9(mut x: ()) -> () {
    return ((x) != (x));
}
pub unsafe fn is_nan_10(mut x: ()) -> () {
    return ((x) != (x));
}
pub unsafe fn is_nan_11(mut x: ()) -> () {
    return ((x) != (x));
}
pub unsafe fn get_error_pct_12() -> () {
    {
        return 1e-04_f32;
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn get_error_pct_13() -> () {
    {
        {
            return 1e-12_f64;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn get_error_pct_14() -> () {
    {
        {
            return 1e-13_f64;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn check_near_15(mut a: (), mut b: (), mut expect: ()) -> () {
    {}
    return (unsafe {
        fptest_close_pct_6(unimplemented!(), expect, (unsafe { get_error_pct_12() }))
    });
}
pub unsafe fn check_near_16(mut a: (), mut b: (), mut expect: ()) -> () {
    {}
    return (unsafe {
        fptest_close_pct_7(unimplemented!(), expect, (unsafe { get_error_pct_13() }))
    });
}
pub unsafe fn check_near_17(mut a: (), mut b: (), mut expect: ()) -> () {
    {}
    return (unsafe {
        fptest_close_pct_8(unimplemented!(), expect, (unsafe { get_error_pct_14() }))
    });
}
pub unsafe fn check_exact_18(mut a: (), mut b: (), mut expect: ()) -> () {
    let mut res: () = unimplemented!();
    if (unsafe { is_nan_9(expect) }) {
        return (unsafe { is_nan_9(res) });
    }
    return ((res) == (expect));
}
pub unsafe fn check_exact_19(mut a: (), mut b: (), mut expect: ()) -> () {
    let mut res: () = unimplemented!();
    if (unsafe { is_nan_10(expect) }) {
        return (unsafe { is_nan_10(res) });
    }
    return ((res) == (expect));
}
pub unsafe fn check_exact_20(mut a: (), mut b: (), mut expect: ()) -> () {
    let mut res: () = unimplemented!();
    if (unsafe { is_nan_11(expect) }) {
        return (unsafe { is_nan_11(res) });
    }
    return ((res) == (expect));
}
pub unsafe fn test_ppc_edge_cases_21() -> () {
    {}
    let mut d1: () = 3.14_f64;
    let mut d0: () = unimplemented!();
    let mut d2: () = unimplemented!();
    let mut verify: FnPtr<fn((), (), ()) -> ()> =
        FnPtr::<fn((), (), ()) -> ()>::new(|mut res: (), mut low: (), mut high: ()| -> () {
            unsafe {
                return (((res) == (low)) || ((res) == (high)))
                    && (((low) <= (res)) && ((res) <= (high)));
            }
        });
    return (unsafe {
        let _res: () = unimplemented!();
        let _low: () = d0;
        let _high: () = d1;
        verify.call(_res, _low, _high)
    }) && (unsafe {
        let _res: () = unimplemented!();
        let _low: () = d1;
        let _high: () = d2;
        verify.call(_res, _low, _high)
    });
}
pub unsafe fn test_ppc_edge_cases_22() -> () {
    {}
    let mut d1: () = 3.14_f64;
    let mut d0: () = unimplemented!();
    let mut d2: () = unimplemented!();
    let mut verify: FnPtr<fn((), (), ()) -> ()> =
        FnPtr::<fn((), (), ()) -> ()>::new(|mut res: (), mut low: (), mut high: ()| -> () {
            unsafe {
                return (((res) == (low)) || ((res) == (high)))
                    && (((low) <= (res)) && ((res) <= (high)));
            }
        });
    return (unsafe {
        let _res: () = unimplemented!();
        let _low: () = d0;
        let _high: () = d1;
        verify.call(_res, _low, _high)
    }) && (unsafe {
        let _res: () = unimplemented!();
        let _low: () = d1;
        let _high: () = d2;
        verify.call(_res, _low, _high)
    });
}
pub unsafe fn test_ppc_edge_cases_23() -> () {
    {}
    let mut d1: () = 3.14_f64;
    let mut d0: () = unimplemented!();
    let mut d2: () = unimplemented!();
    let mut verify: FnPtr<fn((), (), ()) -> ()> =
        FnPtr::<fn((), (), ()) -> ()>::new(|mut res: (), mut low: (), mut high: ()| -> () {
            unsafe {
                return (((res) == (low)) || ((res) == (high)))
                    && (((low) <= (res)) && ((res) <= (high)));
            }
        });
    return (unsafe {
        let _res: () = unimplemented!();
        let _low: () = d0;
        let _high: () = d1;
        verify.call(_res, _low, _high)
    }) && (unsafe {
        let _res: () = unimplemented!();
        let _low: () = d1;
        let _high: () = d2;
        verify.call(_res, _low, _high)
    });
}
pub unsafe fn test_floating_points_24() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let max_v: () = unimplemented!();
    let min_v: () = unimplemented!();
    let denorm_min: () = unimplemented!();
    let inf: () = unimplemented!();
    let qnan: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_floating_points_25() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let max_v: () = unimplemented!();
    let min_v: () = unimplemented!();
    let denorm_min: () = unimplemented!();
    let inf: () = unimplemented!();
    let qnan: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_floating_points_26() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let max_v: () = unimplemented!();
    let min_v: () = unimplemented!();
    let denorm_min: () = unimplemented!();
    let inf: () = unimplemented!();
    let qnan: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
    }
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_27() -> () {
    (unsafe { test_floating_points_24() });
    (unsafe { test_floating_points_25() });
    (unsafe { test_floating_points_26() });
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
    (unsafe { test_27() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
