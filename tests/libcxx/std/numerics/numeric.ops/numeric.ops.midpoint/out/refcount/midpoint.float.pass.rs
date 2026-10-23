extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_float__float_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_double__double_ {}
pub type anon_2 = ();
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long_double__long_double_ {}
pub fn fptest_close_3(mut val: (), mut expected: (), mut eps: ()) -> () {
    let mut zero: () = 0_();
    unimplemented!();
    if (eps == zero) {
        return (val == expected);
    }
    if (val == zero) {
        return (({ abs_4(expected) }) <= eps);
    }
    if (expected == zero) {
        return (({ abs_4(val) }) <= eps);
    }
    return (({ abs_4((val - expected)) }) < eps)
        && ((({ abs_4((val - expected)) }) / ({ abs_4(val) })) < eps);
}
pub fn fptest_close_5(mut val: (), mut expected: (), mut eps: ()) -> () {
    let mut zero: () = 0_();
    unimplemented!();
    if (eps == zero) {
        return (val == expected);
    }
    if (val == zero) {
        return (({ abs_6(expected) }) <= eps);
    }
    if (expected == zero) {
        return (({ abs_6(val) }) <= eps);
    }
    return (({ abs_6((val - expected)) }) < eps)
        && ((({ abs_6((val - expected)) }) / ({ abs_6(val) })) < eps);
}
pub fn fptest_close_7(mut val: (), mut expected: (), mut eps: ()) -> () {
    let mut zero: () = 0_();
    unimplemented!();
    if (eps == zero) {
        return (val == expected);
    }
    if (val == zero) {
        return (({ abs_8(expected) }) <= eps);
    }
    if (expected == zero) {
        return (({ abs_8(val) }) <= eps);
    }
    return (({ abs_8((val - expected)) }) < eps)
        && ((({ abs_8((val - expected)) }) / ({ abs_8(val) })) < eps);
}
pub fn fptest_close_pct_9(mut val: (), mut expected: (), mut percent: ()) -> () {
    unimplemented!();
    let mut eps: () = ((percent / 100_()) * (unimplemented!().read()));
    return ({ fptest_close_3(val, expected, eps) });
}
pub fn fptest_close_pct_10(mut val: (), mut expected: (), mut percent: ()) -> () {
    unimplemented!();
    let mut eps: () = ((percent / 100_()) * (unimplemented!().read()));
    return ({ fptest_close_5(val, expected, eps) });
}
pub fn fptest_close_pct_11(mut val: (), mut expected: (), mut percent: ()) -> () {
    unimplemented!();
    let mut eps: () = ((percent / 100_()) * (unimplemented!().read()));
    return ({ fptest_close_7(val, expected, eps) });
}
pub fn is_nan_12(mut x: ()) -> () {
    return (x != x);
}
pub fn is_nan_13(mut x: ()) -> () {
    return (x != x);
}
pub fn is_nan_14(mut x: ()) -> () {
    return (x != x);
}
pub fn get_error_pct_15() -> () {
    {
        return 1e-04_f32;
    }
    panic!("ub: non-void function does not return a value")
}
pub fn get_error_pct_16() -> () {
    {
        {
            return 1e-12_f64;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub fn get_error_pct_17() -> () {
    {
        {
            return 1e-13_f64;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub fn check_near_18(mut a: (), mut b: (), mut expect: ()) -> () {
    {}
    return ({ fptest_close_pct_9(unimplemented!(), expect, ({ get_error_pct_15() })) });
}
pub fn check_near_19(mut a: (), mut b: (), mut expect: ()) -> () {
    {}
    return ({ fptest_close_pct_10(unimplemented!(), expect, ({ get_error_pct_16() })) });
}
pub fn check_near_20(mut a: (), mut b: (), mut expect: ()) -> () {
    {}
    return ({ fptest_close_pct_11(unimplemented!(), expect, ({ get_error_pct_17() })) });
}
pub fn check_exact_21(mut a: (), mut b: (), mut expect: ()) -> () {
    let mut res: () = unimplemented!();
    if ({ is_nan_12(expect) }) {
        return ({ is_nan_12(res) });
    }
    return (res == expect);
}
pub fn check_exact_22(mut a: (), mut b: (), mut expect: ()) -> () {
    let mut res: () = unimplemented!();
    if ({ is_nan_13(expect) }) {
        return ({ is_nan_13(res) });
    }
    return (res == expect);
}
pub fn check_exact_23(mut a: (), mut b: (), mut expect: ()) -> () {
    let mut res: () = unimplemented!();
    if ({ is_nan_14(expect) }) {
        return ({ is_nan_14(res) });
    }
    return (res == expect);
}
pub fn test_ppc_edge_cases_24() -> () {
    {}
    let mut d1: () = 3.14_f64;
    let mut d0: () = ({ nextafter_25(d1, 2_()) });
    let mut d2: () = ({ nextafter_25(d1, 5_()) });
    let verify: Value<FnPtr<fn((), (), ()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn((), (), ()) -> ()>::new(
            |mut res: (), mut low: (), mut high: ()| -> () {
                {
                    return ((res == low) || (res == high)) && ((low <= res) && (res <= high));
                }
            },
        )));
    return ({
        let _res: () = unimplemented!();
        let _low: () = d0;
        let _high: () = d1;
        (*verify.borrow()).call(_res, _low, _high)
    }) && ({
        let _res: () = unimplemented!();
        let _low: () = d1;
        let _high: () = d2;
        (*verify.borrow()).call(_res, _low, _high)
    });
}
pub fn test_ppc_edge_cases_26() -> () {
    {}
    let mut d1: () = 3.14_f64;
    let mut d0: () = ({ nextafter_27(d1, 2_()) });
    let mut d2: () = ({ nextafter_27(d1, 5_()) });
    let verify: Value<FnPtr<fn((), (), ()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn((), (), ()) -> ()>::new(
            |mut res: (), mut low: (), mut high: ()| -> () {
                {
                    return ((res == low) || (res == high)) && ((low <= res) && (res <= high));
                }
            },
        )));
    return ({
        let _res: () = unimplemented!();
        let _low: () = d0;
        let _high: () = d1;
        (*verify.borrow()).call(_res, _low, _high)
    }) && ({
        let _res: () = unimplemented!();
        let _low: () = d1;
        let _high: () = d2;
        (*verify.borrow()).call(_res, _low, _high)
    });
}
pub fn test_ppc_edge_cases_28() -> () {
    {}
    let mut d1: () = 3.14_f64;
    let mut d0: () = ({ nextafter_29(d1, 2_()) });
    let mut d2: () = ({ nextafter_29(d1, 5_()) });
    let verify: Value<FnPtr<fn((), (), ()) -> ()>> =
        Rc::new(RefCell::new(FnPtr::<fn((), (), ()) -> ()>::new(
            |mut res: (), mut low: (), mut high: ()| -> () {
                {
                    return ((res == low) || (res == high)) && ((low <= res) && (res <= high));
                }
            },
        )));
    return ({
        let _res: () = unimplemented!();
        let _low: () = d0;
        let _high: () = d1;
        (*verify.borrow()).call(_res, _low, _high)
    }) && ({
        let _res: () = unimplemented!();
        let _low: () = d1;
        let _high: () = d2;
        (*verify.borrow()).call(_res, _low, _high)
    });
}
pub fn test_floating_points_30() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max_v: () = ({ std_numeric_limits_float_::max() });
    let mut min_v: () = ({ std_numeric_limits_float_::min() });
    let mut denorm_min: () = ({ std_numeric_limits_float_::denorm_min() });
    let mut inf: () = ({ std_numeric_limits_float_::infinity() });
    let mut qnan: () = ({ std_numeric_limits_float_::quiet_NaN() });
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
pub fn test_floating_points_31() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max_v: () = ({ std_numeric_limits_double_::max() });
    let mut min_v: () = ({ std_numeric_limits_double_::min() });
    let mut denorm_min: () = ({ std_numeric_limits_double_::denorm_min() });
    let mut inf: () = ({ std_numeric_limits_double_::infinity() });
    let mut qnan: () = ({ std_numeric_limits_double_::quiet_NaN() });
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
pub fn test_floating_points_32() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max_v: () = ({ std_numeric_limits_long_double_::max() });
    let mut min_v: () = ({ std_numeric_limits_long_double_::min() });
    let mut denorm_min: () = ({ std_numeric_limits_long_double_::denorm_min() });
    let mut inf: () = ({ std_numeric_limits_long_double_::infinity() });
    let mut qnan: () = ({ std_numeric_limits_long_double_::quiet_NaN() });
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
pub fn test_33() -> () {
    ({ test_floating_points_30() });
    ({ test_floating_points_31() });
    ({ test_floating_points_32() });
    return true;
}
pub fn main() {
    let size: usize = ::std::env::args().map(|arg| arg.len() + 1).sum();
    let args: Value<Box<[i8]>> = Rc::new(RefCell::new(vec![0; size].into_boxed_slice()));
    let args_ptr: Ptr<i8> = args.as_pointer();
    let mut off = 0;
    let argv: Value<Box<[Ptr<i8>]>> = Rc::new(RefCell::new(
        ::std::env::args()
            .map(|arg| {
                let ptr = args_ptr.offset(off);
                ptr.write_c_bytes(arg.as_bytes());
                off += arg.len() + 1;
                ptr
            })
            .chain(::std::iter::once(Ptr::null()))
            .collect(),
    ));
    __cpp2rust_init_globals();
    ::std::process::exit(main_0(::std::env::args().len() as i32, argv.as_pointer()));
}
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_33() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
