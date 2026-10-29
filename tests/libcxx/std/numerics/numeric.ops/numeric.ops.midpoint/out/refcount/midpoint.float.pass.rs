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
        return (unimplemented!() <= eps);
    }
    if (expected == zero) {
        return (unimplemented!() <= eps);
    }
    return (unimplemented!() < eps) && ((unimplemented!() / unimplemented!()) < eps);
}
pub fn fptest_close_4(mut val: (), mut expected: (), mut eps: ()) -> () {
    let mut zero: () = 0_();
    unimplemented!();
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
pub fn fptest_close_5(mut val: (), mut expected: (), mut eps: ()) -> () {
    let mut zero: () = 0_();
    unimplemented!();
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
pub fn fptest_close_pct_6(mut val: (), mut expected: (), mut percent: ()) -> () {
    unimplemented!();
    let mut eps: () = ((percent / 100_()) * (unimplemented!().read()));
    return ({ fptest_close_3(val, expected, eps) });
}
pub fn fptest_close_pct_7(mut val: (), mut expected: (), mut percent: ()) -> () {
    unimplemented!();
    let mut eps: () = ((percent / 100_()) * (unimplemented!().read()));
    return ({ fptest_close_4(val, expected, eps) });
}
pub fn fptest_close_pct_8(mut val: (), mut expected: (), mut percent: ()) -> () {
    unimplemented!();
    let mut eps: () = ((percent / 100_()) * (unimplemented!().read()));
    return ({ fptest_close_5(val, expected, eps) });
}
pub fn is_nan_9(mut x: ()) -> () {
    return (x != x);
}
pub fn is_nan_10(mut x: ()) -> () {
    return (x != x);
}
pub fn is_nan_11(mut x: ()) -> () {
    return (x != x);
}
pub fn get_error_pct_12() -> () {
    {
        return 1e-04_f32;
    }
    panic!("ub: non-void function does not return a value")
}
pub fn get_error_pct_13() -> () {
    {
        {
            return 1e-12_f64;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub fn get_error_pct_14() -> () {
    {
        {
            return 1e-13_f64;
        }
    }
    panic!("ub: non-void function does not return a value")
}
pub fn check_near_15(mut a: (), mut b: (), mut expect: ()) -> () {
    {}
    return ({ fptest_close_pct_6(unimplemented!(), expect, ({ get_error_pct_12() })) });
}
pub fn check_near_16(mut a: (), mut b: (), mut expect: ()) -> () {
    {}
    return ({ fptest_close_pct_7(unimplemented!(), expect, ({ get_error_pct_13() })) });
}
pub fn check_near_17(mut a: (), mut b: (), mut expect: ()) -> () {
    {}
    return ({ fptest_close_pct_8(unimplemented!(), expect, ({ get_error_pct_14() })) });
}
pub fn check_exact_18(mut a: (), mut b: (), mut expect: ()) -> () {
    let mut res: () = unimplemented!();
    if ({ is_nan_9(expect) }) {
        return ({ is_nan_9(res) });
    }
    return (res == expect);
}
pub fn check_exact_19(mut a: (), mut b: (), mut expect: ()) -> () {
    let mut res: () = unimplemented!();
    if ({ is_nan_10(expect) }) {
        return ({ is_nan_10(res) });
    }
    return (res == expect);
}
pub fn check_exact_20(mut a: (), mut b: (), mut expect: ()) -> () {
    let mut res: () = unimplemented!();
    if ({ is_nan_11(expect) }) {
        return ({ is_nan_11(res) });
    }
    return (res == expect);
}
pub fn test_ppc_edge_cases_21() -> () {
    {}
    let mut d1: () = 3.14_f64;
    let mut d0: () = unimplemented!();
    let mut d2: () = unimplemented!();
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
pub fn test_ppc_edge_cases_22() -> () {
    {}
    let mut d1: () = 3.14_f64;
    let mut d0: () = unimplemented!();
    let mut d2: () = unimplemented!();
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
pub fn test_ppc_edge_cases_23() -> () {
    {}
    let mut d1: () = 3.14_f64;
    let mut d0: () = unimplemented!();
    let mut d2: () = unimplemented!();
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
pub fn test_floating_points_24() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max_v: () = unimplemented!();
    let mut min_v: () = unimplemented!();
    let mut denorm_min: () = unimplemented!();
    let mut inf: () = unimplemented!();
    let mut qnan: () = unimplemented!();
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
pub fn test_floating_points_25() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max_v: () = unimplemented!();
    let mut min_v: () = unimplemented!();
    let mut denorm_min: () = unimplemented!();
    let mut inf: () = unimplemented!();
    let mut qnan: () = unimplemented!();
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
pub fn test_floating_points_26() -> () {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut max_v: () = unimplemented!();
    let mut min_v: () = unimplemented!();
    let mut denorm_min: () = unimplemented!();
    let mut inf: () = unimplemented!();
    let mut qnan: () = unimplemented!();
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
pub fn test_27() -> () {
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
fn main_0(_a0: i32, _a1: Ptr<Ptr<i8>>) -> () {
    ({ test_27() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub fn __cpp2rust_init_globals() {}
