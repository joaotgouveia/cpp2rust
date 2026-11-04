extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_float__float_ {}
impl CppMove for test_macros_detail_is_same_float__float_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_double__double_ {}
impl CppMove for test_macros_detail_is_same_double__double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long_double__long_double_ {}
impl CppMove for test_macros_detail_is_same_long_double__long_double_ {
    fn cpp_move(src: Ptr<Self>) -> Self {
        src.read()
    }
}
pub fn fptest_close_3(mut val: f32, mut expected: f32, mut eps: f32) -> bool {
    let mut zero: f32 = 0_f32;
    assert!((eps >= zero));
    if (eps == zero) {
        return (val == expected);
    }
    if (val == zero) {
        return (({ libcc2rs::abs_refcount(expected) }) <= eps);
    }
    if (expected == zero) {
        return (({ libcc2rs::abs_refcount(val) }) <= eps);
    }
    return (({ libcc2rs::abs_refcount((val - expected)) }) < eps)
        && ((({ libcc2rs::abs_refcount((val - expected)) }) / ({ libcc2rs::abs_refcount(val) }))
            < eps);
}
pub fn fptest_close_4(mut val: f64, mut expected: f64, mut eps: f64) -> bool {
    let mut zero: f64 = 0_f64;
    assert!((eps >= zero));
    if (eps == zero) {
        return (val == expected);
    }
    if (val == zero) {
        return (({ libcc2rs::abs_refcount(expected) }) <= eps);
    }
    if (expected == zero) {
        return (({ libcc2rs::abs_refcount(val) }) <= eps);
    }
    return (({ libcc2rs::abs_refcount((val - expected)) }) < eps)
        && ((({ libcc2rs::abs_refcount((val - expected)) }) / ({ libcc2rs::abs_refcount(val) }))
            < eps);
}
pub fn fptest_close_5(mut val: f64, mut expected: f64, mut eps: f64) -> bool {
    let mut zero: f64 = 0_f64;
    assert!((eps >= zero));
    if (eps == zero) {
        return (val == expected);
    }
    if (val == zero) {
        return (({ libcc2rs::abs_refcount(expected) }) <= eps);
    }
    if (expected == zero) {
        return (({ libcc2rs::abs_refcount(val) }) <= eps);
    }
    return (({ libcc2rs::abs_refcount((val - expected)) }) < eps)
        && ((({ libcc2rs::abs_refcount((val - expected)) }) / ({ libcc2rs::abs_refcount(val) }))
            < eps);
}
pub fn fptest_close_pct_6(mut val: f32, mut expected: f32, mut percent: f32) -> bool {
    assert!((percent >= 0_f32));
    let mut eps: f32 = ((percent / 100_f32)
        * (match (
            Rc::new(RefCell::new(({ libcc2rs::abs_refcount(val) }))).as_pointer(),
            Rc::new(RefCell::new(({ libcc2rs::abs_refcount(expected) }))).as_pointer(),
        ) {
            (__tmp_0, __tmp_1) => {
                if CppLess::less(&(__tmp_0).clone(), &(__tmp_1).clone()) {
                    (__tmp_1).clone()
                } else {
                    (__tmp_0).clone()
                }
            }
        }
        .read()));
    return ({ fptest_close_3(val, expected, eps) });
}
pub fn fptest_close_pct_7(mut val: f64, mut expected: f64, mut percent: f64) -> bool {
    assert!((percent >= 0_f64));
    let mut eps: f64 = ((percent / 100_f64)
        * (match (
            Rc::new(RefCell::new(({ libcc2rs::abs_refcount(val) }))).as_pointer(),
            Rc::new(RefCell::new(({ libcc2rs::abs_refcount(expected) }))).as_pointer(),
        ) {
            (__tmp_2, __tmp_3) => {
                if CppLess::less(&(__tmp_2).clone(), &(__tmp_3).clone()) {
                    (__tmp_3).clone()
                } else {
                    (__tmp_2).clone()
                }
            }
        }
        .read()));
    return ({ fptest_close_4(val, expected, eps) });
}
pub fn fptest_close_pct_8(mut val: f64, mut expected: f64, mut percent: f64) -> bool {
    assert!((percent >= 0_f64));
    let mut eps: f64 = ((percent / 100_f64)
        * (match (
            Rc::new(RefCell::new(({ libcc2rs::abs_refcount(val) }))).as_pointer(),
            Rc::new(RefCell::new(({ libcc2rs::abs_refcount(expected) }))).as_pointer(),
        ) {
            (__tmp_4, __tmp_5) => {
                if CppLess::less(&(__tmp_4).clone(), &(__tmp_5).clone()) {
                    (__tmp_5).clone()
                } else {
                    (__tmp_4).clone()
                }
            }
        }
        .read()));
    return ({ fptest_close_5(val, expected, eps) });
}
pub fn is_nan_9(mut x: f32) -> bool {
    return (x != x);
}
pub fn is_nan_10(mut x: f64) -> bool {
    return (x != x);
}
pub fn is_nan_11(mut x: f64) -> bool {
    return (x != x);
}
pub fn get_error_pct_12() -> f32 {
    {
        return 1e-04_f32;
    }
    panic!("ub: non-void function does not return a value")
}
pub fn get_error_pct_13() -> f64 {
    {
        {
            return 1e-12_f64;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub fn get_error_pct_14() -> f64 {
    {
        {
            return 1e-13_f64;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub fn check_near_15(mut a: f32, mut b: f32, mut expect: f32) -> bool {
    {}
    return ({ fptest_close_pct_6(({ midpoint_16(a, b) }), expect, ({ get_error_pct_12() })) });
}
pub fn check_near_17(mut a: f64, mut b: f64, mut expect: f64) -> bool {
    {}
    return ({ fptest_close_pct_7(({ midpoint_18(a, b) }), expect, ({ get_error_pct_13() })) });
}
pub fn check_near_19(mut a: f64, mut b: f64, mut expect: f64) -> bool {
    {}
    return ({ fptest_close_pct_8(({ midpoint_20(a, b) }), expect, ({ get_error_pct_14() })) });
}
pub fn check_exact_21(mut a: f32, mut b: f32, mut expect: f32) -> bool {
    let mut res: f32 = ({ midpoint_16(a, b) });
    if ({ is_nan_9(expect) }) {
        return ({ is_nan_9(res) });
    }
    return (res == expect);
}
pub fn check_exact_22(mut a: f64, mut b: f64, mut expect: f64) -> bool {
    let mut res: f64 = ({ midpoint_18(a, b) });
    if ({ is_nan_10(expect) }) {
        return ({ is_nan_10(res) });
    }
    return (res == expect);
}
pub fn check_exact_23(mut a: f64, mut b: f64, mut expect: f64) -> bool {
    let mut res: f64 = ({ midpoint_20(a, b) });
    if ({ is_nan_11(expect) }) {
        return ({ is_nan_11(res) });
    }
    return (res == expect);
}
pub fn test_ppc_edge_cases_24() -> bool {
    {}
    let mut d1: f32 = (3.14_f64 as f32);
    let mut d0: f32 = ({ nextafter_25(d1, 2_f32) });
    let mut d2: f32 = ({ nextafter_25(d1, 5_f32) });
    let verify: Value<FnPtr<fn(f32, f32, f32) -> bool>> =
        Rc::new(RefCell::new(FnPtr::<fn(f32, f32, f32) -> bool>::new(
            |mut res: f32, mut low: f32, mut high: f32| -> bool {
                {
                    return ((res == low) || (res == high)) && ((low <= res) && (res <= high));
                }
            },
        )));
    return ({
        let _res: f32 = ({ midpoint_16(d0, d1) });
        let _low: f32 = d0;
        let _high: f32 = d1;
        (*verify.borrow()).call(_res, _low, _high)
    }) && ({
        let _res: f32 = ({ midpoint_16(d1, d2) });
        let _low: f32 = d1;
        let _high: f32 = d2;
        (*verify.borrow()).call(_res, _low, _high)
    });
}
pub fn test_ppc_edge_cases_26() -> bool {
    {}
    let mut d1: f64 = 3.14_f64;
    let mut d0: f64 = ({ nextafter_27(d1, 2_f64) });
    let mut d2: f64 = ({ nextafter_27(d1, 5_f64) });
    let verify: Value<FnPtr<fn(f64, f64, f64) -> bool>> =
        Rc::new(RefCell::new(FnPtr::<fn(f64, f64, f64) -> bool>::new(
            |mut res: f64, mut low: f64, mut high: f64| -> bool {
                {
                    return ((res == low) || (res == high)) && ((low <= res) && (res <= high));
                }
            },
        )));
    return ({
        let _res: f64 = ({ midpoint_18(d0, d1) });
        let _low: f64 = d0;
        let _high: f64 = d1;
        (*verify.borrow()).call(_res, _low, _high)
    }) && ({
        let _res: f64 = ({ midpoint_18(d1, d2) });
        let _low: f64 = d1;
        let _high: f64 = d2;
        (*verify.borrow()).call(_res, _low, _high)
    });
}
pub fn test_ppc_edge_cases_28() -> bool {
    {}
    let mut d1: f64 = 3.14_f64;
    let mut d0: f64 = ({ nextafter_29(d1, 2_f64) });
    let mut d2: f64 = ({ nextafter_29(d1, 5_f64) });
    let verify: Value<FnPtr<fn(f64, f64, f64) -> bool>> =
        Rc::new(RefCell::new(FnPtr::<fn(f64, f64, f64) -> bool>::new(
            |mut res: f64, mut low: f64, mut high: f64| -> bool {
                {
                    return ((res == low) || (res == high)) && ((low <= res) && (res <= high));
                }
            },
        )));
    return ({
        let _res: f64 = ({ midpoint_20(d0, d1) });
        let _low: f64 = d0;
        let _high: f64 = d1;
        (*verify.borrow()).call(_res, _low, _high)
    }) && ({
        let _res: f64 = ({ midpoint_20(d1, d2) });
        let _low: f64 = d1;
        let _high: f64 = d2;
        (*verify.borrow()).call(_res, _low, _high)
    });
}
pub fn test_floating_points_30() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max_v: f32 = 3.4028235e+38_f32;
    let mut min_v: f32 = 1.1754944e-38_f32;
    let mut denorm_min: f32 = 1e-45_f32;
    let mut inf: f32 = f32::INFINITY;
    let mut qnan: f32 = f32::from_bits(0x7fc00000);
    assert!(({ check_exact_21(0_f32, 0_f32, 0_f32,) }));
    assert!(({ check_exact_21(2_f32, 4_f32, 3_f32,) }));
    assert!(({ check_exact_21(4_f32, 2_f32, 3_f32,) }));
    assert!(({ check_exact_21(3_f32, 4_f32, (3.5_f64 as f32),) }));
    assert!(({ check_exact_21(0_f32, (0.4_f64 as f32), (0.2_f64 as f32),) }));
    assert!(({ check_exact_21((-2_i32 as f32), (-4_i32 as f32), (-3_i32 as f32),) }));
    assert!(({ check_exact_21((-2_i32 as f32), 2_f32, 0_f32,) }));
    assert!(({ check_exact_21(2_f32, (-2_i32 as f32), 0_f32,) }));
    assert!(
        ({
            let _a: f32 = inf;
            let _b: f32 = inf;
            let _expect: f32 = inf;
            check_exact_21(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f32 = -inf;
            let _b: f32 = -inf;
            let _expect: f32 = -inf;
            check_exact_21(_a, _b, _expect)
        })
    );
    {
        assert!(
            ({
                let _a: f32 = inf;
                let _b: f32 = -inf;
                check_exact_21(_a, _b, qnan)
            })
        );
    }
    assert!(
        ({
            let _a: f32 = inf;
            let _expect: f32 = inf;
            check_exact_21(_a, 0_f32, _expect)
        })
    );
    {
        assert!(
            ({
                let _a: f32 = qnan;
                let _expect: f32 = qnan;
                check_exact_21(_a, 0_f32, _expect)
            })
        );
        assert!(
            ({
                let _a: f32 = qnan;
                let _b: f32 = qnan;
                let _expect: f32 = qnan;
                check_exact_21(_a, _b, _expect)
            })
        );
    }
    assert!(({ check_exact_21(denorm_min, 0_f32, 0_f32,) }));
    assert!(
        ({
            let _a: f32 = denorm_min;
            let _b: f32 = denorm_min;
            let _expect: f32 = denorm_min;
            check_exact_21(_a, _b, _expect)
        })
    );
    assert!(({ check_near_15((1.3_f64 as f32), (11.4_f64 as f32), (6.35_f64 as f32),) }));
    assert!(({ check_near_15((11.33_f64 as f32), (31.45_f64 as f32), (21.39_f64 as f32),) }));
    assert!(({ check_near_15((-1.3_f64 as f32), (11.4_f64 as f32), (5.05_f64 as f32),) }));
    assert!(({ check_near_15((11.4_f64 as f32), (-1.3_f64 as f32), (5.05_f64 as f32),) }));
    assert!(
        ({
            check_near_15(
                (11.2345_f64 as f32),
                (14.5432_f64 as f32),
                (12.88885_f64 as f32),
            )
        })
    );
    assert!(
        ({
            check_near_15(
                (2.718281828459045_f64 as f32),
                (3.141592653589793_f64 as f32),
                (2.9299372410244193_f64 as f32),
            )
        })
    );
    assert!(
        ({
            let _a: f32 = max_v;
            let _expect: f32 = (max_v / 2_f32);
            check_near_15(_a, 0_f32, _expect)
        })
    );
    assert!(
        ({
            let _b: f32 = max_v;
            let _expect: f32 = (max_v / 2_f32);
            check_near_15(0_f32, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f32 = min_v;
            let _expect: f32 = (min_v / 2_f32);
            check_near_15(_a, 0_f32, _expect)
        })
    );
    assert!(
        ({
            let _b: f32 = min_v;
            let _expect: f32 = (min_v / 2_f32);
            check_near_15(0_f32, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f32 = max_v;
            let _b: f32 = max_v;
            let _expect: f32 = max_v;
            check_near_15(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f32 = min_v;
            let _b: f32 = min_v;
            let _expect: f32 = min_v;
            check_near_15(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f32 = max_v;
            let _expect: f32 = (max_v / 2_f32);
            check_near_15(_a, min_v, _expect)
        })
    );
    assert!(
        ({
            let _b: f32 = max_v;
            let _expect: f32 = (max_v / 2_f32);
            check_near_15(min_v, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f32 = (((max_v as f64) * 0.75_f64) as f32);
            let _b: f32 = (((max_v as f64) * 0.5_f64) as f32);
            let _expect: f32 = (((max_v as f64) * 0.625_f64) as f32);
            check_near_15(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f32 = (((max_v as f64) * 0.5_f64) as f32);
            let _b: f32 = (((max_v as f64) * 0.75_f64) as f32);
            let _expect: f32 = (((max_v as f64) * 0.625_f64) as f32);
            check_near_15(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f32 = (min_v * 2_f32);
            let _b: f32 = (min_v * 8_f32);
            let _expect: f32 = (min_v * 5_f32);
            check_near_15(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f32 = (((max_v as f64) * 0.75_f64) as f32);
            let _b: f32 = (((max_v as f64) * -0.5_f64) as f32);
            let _expect: f32 = (((max_v as f64) * 0.125_f64) as f32);
            check_near_15(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f32 = (((max_v as f64) * -0.75_f64) as f32);
            let _b: f32 = (((max_v as f64) * 0.5_f64) as f32);
            let _expect: f32 = (((max_v as f64) * -0.125_f64) as f32);
            check_near_15(_a, _b, _expect)
        })
    );
    assert!(({ test_ppc_edge_cases_24() }));
    return true;
}
pub fn test_floating_points_31() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max_v: f64 = 1.7976931348623157e+308_f64;
    let mut min_v: f64 = 2.2250738585072014e-308_f64;
    let mut denorm_min: f64 = 5e-324_f64;
    let mut inf: f64 = f64::INFINITY;
    let mut qnan: f64 = f64::from_bits(0x7ff8000000000000);
    assert!(({ check_exact_22(0_f64, 0_f64, 0_f64,) }));
    assert!(({ check_exact_22(2_f64, 4_f64, 3_f64,) }));
    assert!(({ check_exact_22(4_f64, 2_f64, 3_f64,) }));
    assert!(({ check_exact_22(3_f64, 4_f64, 3.5_f64,) }));
    assert!(({ check_exact_22(0_f64, 0.4_f64, 0.2_f64,) }));
    assert!(({ check_exact_22((-2_i32 as f64), (-4_i32 as f64), (-3_i32 as f64),) }));
    assert!(({ check_exact_22((-2_i32 as f64), 2_f64, 0_f64,) }));
    assert!(({ check_exact_22(2_f64, (-2_i32 as f64), 0_f64,) }));
    assert!(
        ({
            let _a: f64 = inf;
            let _b: f64 = inf;
            let _expect: f64 = inf;
            check_exact_22(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = -inf;
            let _b: f64 = -inf;
            let _expect: f64 = -inf;
            check_exact_22(_a, _b, _expect)
        })
    );
    {
        assert!(
            ({
                let _a: f64 = inf;
                let _b: f64 = -inf;
                check_exact_22(_a, _b, qnan)
            })
        );
    }
    assert!(
        ({
            let _a: f64 = inf;
            let _expect: f64 = inf;
            check_exact_22(_a, 0_f64, _expect)
        })
    );
    {
        assert!(
            ({
                let _a: f64 = qnan;
                let _expect: f64 = qnan;
                check_exact_22(_a, 0_f64, _expect)
            })
        );
        assert!(
            ({
                let _a: f64 = qnan;
                let _b: f64 = qnan;
                let _expect: f64 = qnan;
                check_exact_22(_a, _b, _expect)
            })
        );
    }
    assert!(({ check_exact_22(denorm_min, 0_f64, 0_f64,) }));
    assert!(
        ({
            let _a: f64 = denorm_min;
            let _b: f64 = denorm_min;
            let _expect: f64 = denorm_min;
            check_exact_22(_a, _b, _expect)
        })
    );
    assert!(({ check_near_17(1.3_f64, 11.4_f64, 6.35_f64,) }));
    assert!(({ check_near_17(11.33_f64, 31.45_f64, 21.39_f64,) }));
    assert!(({ check_near_17(-1.3_f64, 11.4_f64, 5.05_f64,) }));
    assert!(({ check_near_17(11.4_f64, -1.3_f64, 5.05_f64,) }));
    assert!(({ check_near_17(11.2345_f64, 14.5432_f64, 12.88885_f64,) }));
    assert!(
        ({
            check_near_17(
                2.718281828459045_f64,
                3.141592653589793_f64,
                2.9299372410244193_f64,
            )
        })
    );
    assert!(
        ({
            let _a: f64 = max_v;
            let _expect: f64 = (max_v / 2_f64);
            check_near_17(_a, 0_f64, _expect)
        })
    );
    assert!(
        ({
            let _b: f64 = max_v;
            let _expect: f64 = (max_v / 2_f64);
            check_near_17(0_f64, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = min_v;
            let _expect: f64 = (min_v / 2_f64);
            check_near_17(_a, 0_f64, _expect)
        })
    );
    assert!(
        ({
            let _b: f64 = min_v;
            let _expect: f64 = (min_v / 2_f64);
            check_near_17(0_f64, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = max_v;
            let _b: f64 = max_v;
            let _expect: f64 = max_v;
            check_near_17(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = min_v;
            let _b: f64 = min_v;
            let _expect: f64 = min_v;
            check_near_17(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = max_v;
            let _expect: f64 = (max_v / 2_f64);
            check_near_17(_a, min_v, _expect)
        })
    );
    assert!(
        ({
            let _b: f64 = max_v;
            let _expect: f64 = (max_v / 2_f64);
            check_near_17(min_v, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (max_v * 0.75_f64);
            let _b: f64 = (max_v * 0.5_f64);
            let _expect: f64 = (max_v * 0.625_f64);
            check_near_17(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (max_v * 0.5_f64);
            let _b: f64 = (max_v * 0.75_f64);
            let _expect: f64 = (max_v * 0.625_f64);
            check_near_17(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (min_v * 2_f64);
            let _b: f64 = (min_v * 8_f64);
            let _expect: f64 = (min_v * 5_f64);
            check_near_17(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (max_v * 0.75_f64);
            let _b: f64 = (max_v * -0.5_f64);
            let _expect: f64 = (max_v * 0.125_f64);
            check_near_17(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (max_v * -0.75_f64);
            let _b: f64 = (max_v * 0.5_f64);
            let _expect: f64 = (max_v * -0.125_f64);
            check_near_17(_a, _b, _expect)
        })
    );
    assert!(({ test_ppc_edge_cases_26() }));
    return true;
}
pub fn test_floating_points_32() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max_v: f64 = f64::INFINITY;
    let mut min_v: f64 = 0_f64;
    let mut denorm_min: f64 = 0_f64;
    let mut inf: f64 = f64::INFINITY;
    let mut qnan: f64 = f64::from_bits(0x7ff8000000000000);
    assert!(({ check_exact_23(0_f64, 0_f64, 0_f64,) }));
    assert!(({ check_exact_23(2_f64, 4_f64, 3_f64,) }));
    assert!(({ check_exact_23(4_f64, 2_f64, 3_f64,) }));
    assert!(({ check_exact_23(3_f64, 4_f64, 3.5_f64,) }));
    assert!(({ check_exact_23(0_f64, 0.4_f64, 0.2_f64,) }));
    assert!(({ check_exact_23((-2_i32 as f64), (-4_i32 as f64), (-3_i32 as f64),) }));
    assert!(({ check_exact_23((-2_i32 as f64), 2_f64, 0_f64,) }));
    assert!(({ check_exact_23(2_f64, (-2_i32 as f64), 0_f64,) }));
    assert!(
        ({
            let _a: f64 = inf;
            let _b: f64 = inf;
            let _expect: f64 = inf;
            check_exact_23(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = -inf;
            let _b: f64 = -inf;
            let _expect: f64 = -inf;
            check_exact_23(_a, _b, _expect)
        })
    );
    {
        assert!(
            ({
                let _a: f64 = inf;
                let _b: f64 = -inf;
                check_exact_23(_a, _b, qnan)
            })
        );
    }
    assert!(
        ({
            let _a: f64 = inf;
            let _expect: f64 = inf;
            check_exact_23(_a, 0_f64, _expect)
        })
    );
    {
        assert!(
            ({
                let _a: f64 = qnan;
                let _expect: f64 = qnan;
                check_exact_23(_a, 0_f64, _expect)
            })
        );
        assert!(
            ({
                let _a: f64 = qnan;
                let _b: f64 = qnan;
                let _expect: f64 = qnan;
                check_exact_23(_a, _b, _expect)
            })
        );
    }
    assert!(({ check_exact_23(denorm_min, 0_f64, 0_f64,) }));
    assert!(
        ({
            let _a: f64 = denorm_min;
            let _b: f64 = denorm_min;
            let _expect: f64 = denorm_min;
            check_exact_23(_a, _b, _expect)
        })
    );
    assert!(({ check_near_19(1.3_f64, 11.4_f64, 6.35_f64,) }));
    assert!(({ check_near_19(11.33_f64, 31.45_f64, 21.39_f64,) }));
    assert!(({ check_near_19(-1.3_f64, 11.4_f64, 5.05_f64,) }));
    assert!(({ check_near_19(11.4_f64, -1.3_f64, 5.05_f64,) }));
    assert!(({ check_near_19(11.2345_f64, 14.5432_f64, 12.88885_f64,) }));
    assert!(
        ({
            check_near_19(
                2.718281828459045_f64,
                3.141592653589793_f64,
                2.9299372410244193_f64,
            )
        })
    );
    assert!(
        ({
            let _a: f64 = max_v;
            let _expect: f64 = (max_v / 2_f64);
            check_near_19(_a, 0_f64, _expect)
        })
    );
    assert!(
        ({
            let _b: f64 = max_v;
            let _expect: f64 = (max_v / 2_f64);
            check_near_19(0_f64, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = min_v;
            let _expect: f64 = (min_v / 2_f64);
            check_near_19(_a, 0_f64, _expect)
        })
    );
    assert!(
        ({
            let _b: f64 = min_v;
            let _expect: f64 = (min_v / 2_f64);
            check_near_19(0_f64, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = max_v;
            let _b: f64 = max_v;
            let _expect: f64 = max_v;
            check_near_19(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = min_v;
            let _b: f64 = min_v;
            let _expect: f64 = min_v;
            check_near_19(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = max_v;
            let _expect: f64 = (max_v / 2_f64);
            check_near_19(_a, min_v, _expect)
        })
    );
    assert!(
        ({
            let _b: f64 = max_v;
            let _expect: f64 = (max_v / 2_f64);
            check_near_19(min_v, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (max_v * 0.75_f64);
            let _b: f64 = (max_v * 0.5_f64);
            let _expect: f64 = (max_v * 0.625_f64);
            check_near_19(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (max_v * 0.5_f64);
            let _b: f64 = (max_v * 0.75_f64);
            let _expect: f64 = (max_v * 0.625_f64);
            check_near_19(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (min_v * 2_f64);
            let _b: f64 = (min_v * 8_f64);
            let _expect: f64 = (min_v * 5_f64);
            check_near_19(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (max_v * 0.75_f64);
            let _b: f64 = (max_v * -0.5_f64);
            let _expect: f64 = (max_v * 0.125_f64);
            check_near_19(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (max_v * -0.75_f64);
            let _b: f64 = (max_v * 0.5_f64);
            let _expect: f64 = (max_v * -0.125_f64);
            check_near_19(_a, _b, _expect)
        })
    );
    assert!(({ test_ppc_edge_cases_28() }));
    return true;
}
pub fn test_33() -> bool {
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> i32 {
    ({ test_33() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
