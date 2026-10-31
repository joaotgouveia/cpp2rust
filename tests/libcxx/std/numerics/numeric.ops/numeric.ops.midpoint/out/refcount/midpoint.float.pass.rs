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
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_double__double_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct test_macros_detail_is_same_long_double__long_double_ {}
pub fn fptest_close_3(mut val: f32, mut expected: f32, mut eps: f32) -> bool {
    let mut zero: f32 = 0_f32;
    assert!((eps >= zero));
    if (eps == zero) {
        return (val == expected);
    }
    if (val == zero) {
        return (unimplemented!() <= eps);
    }
    if (expected == zero) {
        return (unimplemented!() <= eps);
    }
    return (unimplemented!() < eps) && ((unimplemented!() / unimplemented!()) < eps);
}
pub fn fptest_close_4(mut val: f64, mut expected: f64, mut eps: f64) -> bool {
    let mut zero: f64 = 0_f64;
    assert!((eps >= zero));
    if (eps == zero) {
        return (val == expected);
    }
    if (val == zero) {
        return (unimplemented!() <= eps);
    }
    if (expected == zero) {
        return (unimplemented!() <= eps);
    }
    return (unimplemented!() < eps) && ((unimplemented!() / unimplemented!()) < eps);
}
pub fn fptest_close_5(mut val: f64, mut expected: f64, mut eps: f64) -> bool {
    let mut zero: f64 = 0_f64;
    assert!((eps >= zero));
    if (eps == zero) {
        return (val == expected);
    }
    if (val == zero) {
        return (unimplemented!() <= eps);
    }
    if (expected == zero) {
        return (unimplemented!() <= eps);
    }
    return (unimplemented!() < eps) && ((unimplemented!() / unimplemented!()) < eps);
}
pub fn fptest_close_pct_6(mut val: f32, mut expected: f32, mut percent: f32) -> bool {
    assert!((percent >= 0_f32));
    let mut eps: f32 = ((percent / 100_f32) * {
        let __tmp_0: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
        let __tmp_1: Value<f32> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let __a = __tmp_0.as_pointer();
            let __b = __tmp_1.as_pointer();
            if CppLess::less(&__a, &__b) { __b } else { __a }
        }
        .read())
    });
    return ({ fptest_close_3(val, expected, eps) });
}
pub fn fptest_close_pct_7(mut val: f64, mut expected: f64, mut percent: f64) -> bool {
    assert!((percent >= 0_f64));
    let mut eps: f64 = ((percent / 100_f64) * {
        let __tmp_2: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
        let __tmp_3: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let __a = __tmp_2.as_pointer();
            let __b = __tmp_3.as_pointer();
            if CppLess::less(&__a, &__b) { __b } else { __a }
        }
        .read())
    });
    return ({ fptest_close_4(val, expected, eps) });
}
pub fn fptest_close_pct_8(mut val: f64, mut expected: f64, mut percent: f64) -> bool {
    assert!((percent >= 0_f64));
    let mut eps: f64 = ((percent / 100_f64) * {
        let __tmp_4: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
        let __tmp_5: Value<f64> = Rc::new(RefCell::new(unimplemented!()));
        ({
            let __a = __tmp_4.as_pointer();
            let __b = __tmp_5.as_pointer();
            if CppLess::less(&__a, &__b) { __b } else { __a }
        }
        .read())
    });
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
    return ({ fptest_close_pct_6(unimplemented!(), expect, ({ get_error_pct_12() })) });
}
pub fn check_near_16(mut a: f64, mut b: f64, mut expect: f64) -> bool {
    {}
    return ({ fptest_close_pct_7(unimplemented!(), expect, ({ get_error_pct_13() })) });
}
pub fn check_near_17(mut a: f64, mut b: f64, mut expect: f64) -> bool {
    {}
    return ({ fptest_close_pct_8(unimplemented!(), expect, ({ get_error_pct_14() })) });
}
pub fn check_exact_18(mut a: f32, mut b: f32, mut expect: f32) -> bool {
    let mut res: f32 = unimplemented!();
    if ({ is_nan_9(expect) }) {
        return ({ is_nan_9(res) });
    }
    return (res == expect);
}
pub fn check_exact_19(mut a: f64, mut b: f64, mut expect: f64) -> bool {
    let mut res: f64 = unimplemented!();
    if ({ is_nan_10(expect) }) {
        return ({ is_nan_10(res) });
    }
    return (res == expect);
}
pub fn check_exact_20(mut a: f64, mut b: f64, mut expect: f64) -> bool {
    let mut res: f64 = unimplemented!();
    if ({ is_nan_11(expect) }) {
        return ({ is_nan_11(res) });
    }
    return (res == expect);
}
pub fn test_ppc_edge_cases_21() -> bool {
    {}
    let mut d1: f32 = (3.14_f64 as f32);
    let mut d0: f32 = unimplemented!();
    let mut d2: f32 = unimplemented!();
    let verify: Value<FnPtr<fn(f32, f32, f32) -> bool>> =
        Rc::new(RefCell::new(FnPtr::<fn(f32, f32, f32) -> bool>::new(
            |mut res: f32, mut low: f32, mut high: f32| -> bool {
                {
                    return ((res == low) || (res == high)) && ((low <= res) && (res <= high));
                }
            },
        )));
    return ({
        let _res: f32 = unimplemented!();
        let _low: f32 = d0;
        let _high: f32 = d1;
        (*verify.borrow()).call(_res, _low, _high)
    }) && ({
        let _res: f32 = unimplemented!();
        let _low: f32 = d1;
        let _high: f32 = d2;
        (*verify.borrow()).call(_res, _low, _high)
    });
}
pub fn test_ppc_edge_cases_22() -> bool {
    {}
    let mut d1: f64 = 3.14_f64;
    let mut d0: f64 = unimplemented!();
    let mut d2: f64 = unimplemented!();
    let verify: Value<FnPtr<fn(f64, f64, f64) -> bool>> =
        Rc::new(RefCell::new(FnPtr::<fn(f64, f64, f64) -> bool>::new(
            |mut res: f64, mut low: f64, mut high: f64| -> bool {
                {
                    return ((res == low) || (res == high)) && ((low <= res) && (res <= high));
                }
            },
        )));
    return ({
        let _res: f64 = unimplemented!();
        let _low: f64 = d0;
        let _high: f64 = d1;
        (*verify.borrow()).call(_res, _low, _high)
    }) && ({
        let _res: f64 = unimplemented!();
        let _low: f64 = d1;
        let _high: f64 = d2;
        (*verify.borrow()).call(_res, _low, _high)
    });
}
pub fn test_ppc_edge_cases_23() -> bool {
    {}
    let mut d1: f64 = 3.14_f64;
    let mut d0: f64 = unimplemented!();
    let mut d2: f64 = unimplemented!();
    let verify: Value<FnPtr<fn(f64, f64, f64) -> bool>> =
        Rc::new(RefCell::new(FnPtr::<fn(f64, f64, f64) -> bool>::new(
            |mut res: f64, mut low: f64, mut high: f64| -> bool {
                {
                    return ((res == low) || (res == high)) && ((low <= res) && (res <= high));
                }
            },
        )));
    return ({
        let _res: f64 = unimplemented!();
        let _low: f64 = d0;
        let _high: f64 = d1;
        (*verify.borrow()).call(_res, _low, _high)
    }) && ({
        let _res: f64 = unimplemented!();
        let _low: f64 = d1;
        let _high: f64 = d2;
        (*verify.borrow()).call(_res, _low, _high)
    });
}
pub fn test_floating_points_24() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max_v: f32 = f32::MAX;
    let mut min_v: f32 = f32::MIN_POSITIVE;
    let mut denorm_min: f32 = unimplemented!();
    let mut inf: f32 = unimplemented!();
    let mut qnan: f32 = unimplemented!();
    assert!(({ check_exact_18(0_f32, 0_f32, 0_f32,) }));
    assert!(({ check_exact_18(2_f32, 4_f32, 3_f32,) }));
    assert!(({ check_exact_18(4_f32, 2_f32, 3_f32,) }));
    assert!(({ check_exact_18(3_f32, 4_f32, (3.5_f64 as f32),) }));
    assert!(({ check_exact_18(0_f32, (0.4_f64 as f32), (0.2_f64 as f32),) }));
    assert!(({ check_exact_18((-2_i32 as f32), (-4_i32 as f32), (-3_i32 as f32),) }));
    assert!(({ check_exact_18((-2_i32 as f32), 2_f32, 0_f32,) }));
    assert!(({ check_exact_18(2_f32, (-2_i32 as f32), 0_f32,) }));
    assert!(
        ({
            let _a: f32 = inf;
            let _b: f32 = inf;
            let _expect: f32 = inf;
            check_exact_18(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f32 = -inf;
            let _b: f32 = -inf;
            let _expect: f32 = -inf;
            check_exact_18(_a, _b, _expect)
        })
    );
    {
        assert!(
            ({
                let _a: f32 = inf;
                let _b: f32 = -inf;
                check_exact_18(_a, _b, qnan)
            })
        );
    }
    assert!(
        ({
            let _a: f32 = inf;
            let _expect: f32 = inf;
            check_exact_18(_a, 0_f32, _expect)
        })
    );
    {
        assert!(
            ({
                let _a: f32 = qnan;
                let _expect: f32 = qnan;
                check_exact_18(_a, 0_f32, _expect)
            })
        );
        assert!(
            ({
                let _a: f32 = qnan;
                let _b: f32 = qnan;
                let _expect: f32 = qnan;
                check_exact_18(_a, _b, _expect)
            })
        );
    }
    assert!(({ check_exact_18(denorm_min, 0_f32, 0_f32,) }));
    assert!(
        ({
            let _a: f32 = denorm_min;
            let _b: f32 = denorm_min;
            let _expect: f32 = denorm_min;
            check_exact_18(_a, _b, _expect)
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
    assert!(({ test_ppc_edge_cases_21() }));
    return true;
}
pub fn test_floating_points_25() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max_v: f64 = f64::MAX;
    let mut min_v: f64 = f64::MIN_POSITIVE;
    let mut denorm_min: f64 = unimplemented!();
    let mut inf: f64 = unimplemented!();
    let mut qnan: f64 = unimplemented!();
    assert!(({ check_exact_19(0_f64, 0_f64, 0_f64,) }));
    assert!(({ check_exact_19(2_f64, 4_f64, 3_f64,) }));
    assert!(({ check_exact_19(4_f64, 2_f64, 3_f64,) }));
    assert!(({ check_exact_19(3_f64, 4_f64, 3.5_f64,) }));
    assert!(({ check_exact_19(0_f64, 0.4_f64, 0.2_f64,) }));
    assert!(({ check_exact_19((-2_i32 as f64), (-4_i32 as f64), (-3_i32 as f64),) }));
    assert!(({ check_exact_19((-2_i32 as f64), 2_f64, 0_f64,) }));
    assert!(({ check_exact_19(2_f64, (-2_i32 as f64), 0_f64,) }));
    assert!(
        ({
            let _a: f64 = inf;
            let _b: f64 = inf;
            let _expect: f64 = inf;
            check_exact_19(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = -inf;
            let _b: f64 = -inf;
            let _expect: f64 = -inf;
            check_exact_19(_a, _b, _expect)
        })
    );
    {
        assert!(
            ({
                let _a: f64 = inf;
                let _b: f64 = -inf;
                check_exact_19(_a, _b, qnan)
            })
        );
    }
    assert!(
        ({
            let _a: f64 = inf;
            let _expect: f64 = inf;
            check_exact_19(_a, 0_f64, _expect)
        })
    );
    {
        assert!(
            ({
                let _a: f64 = qnan;
                let _expect: f64 = qnan;
                check_exact_19(_a, 0_f64, _expect)
            })
        );
        assert!(
            ({
                let _a: f64 = qnan;
                let _b: f64 = qnan;
                let _expect: f64 = qnan;
                check_exact_19(_a, _b, _expect)
            })
        );
    }
    assert!(({ check_exact_19(denorm_min, 0_f64, 0_f64,) }));
    assert!(
        ({
            let _a: f64 = denorm_min;
            let _b: f64 = denorm_min;
            let _expect: f64 = denorm_min;
            check_exact_19(_a, _b, _expect)
        })
    );
    assert!(({ check_near_16(1.3_f64, 11.4_f64, 6.35_f64,) }));
    assert!(({ check_near_16(11.33_f64, 31.45_f64, 21.39_f64,) }));
    assert!(({ check_near_16(-1.3_f64, 11.4_f64, 5.05_f64,) }));
    assert!(({ check_near_16(11.4_f64, -1.3_f64, 5.05_f64,) }));
    assert!(({ check_near_16(11.2345_f64, 14.5432_f64, 12.88885_f64,) }));
    assert!(
        ({
            check_near_16(
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
            check_near_16(_a, 0_f64, _expect)
        })
    );
    assert!(
        ({
            let _b: f64 = max_v;
            let _expect: f64 = (max_v / 2_f64);
            check_near_16(0_f64, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = min_v;
            let _expect: f64 = (min_v / 2_f64);
            check_near_16(_a, 0_f64, _expect)
        })
    );
    assert!(
        ({
            let _b: f64 = min_v;
            let _expect: f64 = (min_v / 2_f64);
            check_near_16(0_f64, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = max_v;
            let _b: f64 = max_v;
            let _expect: f64 = max_v;
            check_near_16(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = min_v;
            let _b: f64 = min_v;
            let _expect: f64 = min_v;
            check_near_16(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = max_v;
            let _expect: f64 = (max_v / 2_f64);
            check_near_16(_a, min_v, _expect)
        })
    );
    assert!(
        ({
            let _b: f64 = max_v;
            let _expect: f64 = (max_v / 2_f64);
            check_near_16(min_v, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (max_v * 0.75_f64);
            let _b: f64 = (max_v * 0.5_f64);
            let _expect: f64 = (max_v * 0.625_f64);
            check_near_16(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (max_v * 0.5_f64);
            let _b: f64 = (max_v * 0.75_f64);
            let _expect: f64 = (max_v * 0.625_f64);
            check_near_16(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (min_v * 2_f64);
            let _b: f64 = (min_v * 8_f64);
            let _expect: f64 = (min_v * 5_f64);
            check_near_16(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (max_v * 0.75_f64);
            let _b: f64 = (max_v * -0.5_f64);
            let _expect: f64 = (max_v * 0.125_f64);
            check_near_16(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = (max_v * -0.75_f64);
            let _b: f64 = (max_v * 0.5_f64);
            let _expect: f64 = (max_v * -0.125_f64);
            check_near_16(_a, _b, _expect)
        })
    );
    assert!(({ test_ppc_edge_cases_22() }));
    return true;
}
pub fn test_floating_points_26() -> bool {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max_v: f64 = f64::MAX;
    let mut min_v: f64 = f64::MIN_POSITIVE;
    let mut denorm_min: f64 = unimplemented!();
    let mut inf: f64 = unimplemented!();
    let mut qnan: f64 = unimplemented!();
    assert!(({ check_exact_20(0_f64, 0_f64, 0_f64,) }));
    assert!(({ check_exact_20(2_f64, 4_f64, 3_f64,) }));
    assert!(({ check_exact_20(4_f64, 2_f64, 3_f64,) }));
    assert!(({ check_exact_20(3_f64, 4_f64, 3.5_f64,) }));
    assert!(({ check_exact_20(0_f64, 0.4_f64, 0.2_f64,) }));
    assert!(({ check_exact_20((-2_i32 as f64), (-4_i32 as f64), (-3_i32 as f64),) }));
    assert!(({ check_exact_20((-2_i32 as f64), 2_f64, 0_f64,) }));
    assert!(({ check_exact_20(2_f64, (-2_i32 as f64), 0_f64,) }));
    assert!(
        ({
            let _a: f64 = inf;
            let _b: f64 = inf;
            let _expect: f64 = inf;
            check_exact_20(_a, _b, _expect)
        })
    );
    assert!(
        ({
            let _a: f64 = -inf;
            let _b: f64 = -inf;
            let _expect: f64 = -inf;
            check_exact_20(_a, _b, _expect)
        })
    );
    {
        assert!(
            ({
                let _a: f64 = inf;
                let _b: f64 = -inf;
                check_exact_20(_a, _b, qnan)
            })
        );
    }
    assert!(
        ({
            let _a: f64 = inf;
            let _expect: f64 = inf;
            check_exact_20(_a, 0_f64, _expect)
        })
    );
    {
        assert!(
            ({
                let _a: f64 = qnan;
                let _expect: f64 = qnan;
                check_exact_20(_a, 0_f64, _expect)
            })
        );
        assert!(
            ({
                let _a: f64 = qnan;
                let _b: f64 = qnan;
                let _expect: f64 = qnan;
                check_exact_20(_a, _b, _expect)
            })
        );
    }
    assert!(({ check_exact_20(denorm_min, 0_f64, 0_f64,) }));
    assert!(
        ({
            let _a: f64 = denorm_min;
            let _b: f64 = denorm_min;
            let _expect: f64 = denorm_min;
            check_exact_20(_a, _b, _expect)
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
    assert!(({ test_ppc_edge_cases_23() }));
    return true;
}
pub fn test_27() -> bool {
    ({ test_floating_points_24() });
    ({ test_floating_points_25() });
    ({ test_floating_points_26() });
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
    ({ test_27() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub fn __cpp2rust_init_globals() {}
