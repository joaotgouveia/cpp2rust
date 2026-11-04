extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
thread_local!(
    pub static testcases_0: Value<Box<[std_complex_double_]>> = Rc::new(RefCell::new(Box::new([
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
        <std_complex_double_>::default(),
    ])));
);
pub type anon_1 = u32;
pub const anon_1_zero: anon_1 = 0;
pub const anon_1_non_zero: anon_1 = 1;
pub const anon_1_inf: anon_1 = 2;
pub const anon_1_NaN: anon_1 = 3;
pub const anon_1_non_zero_nan: anon_1 = 4;
pub fn classify_2(mut x: f64) -> i32 {
    if (x == 0_f64) {
        return (anon_1_zero as i32);
    }
    if ({ isinf_3(x) }) {
        return (anon_1_inf as i32);
    }
    if ({ isnan_4(x) }) {
        return (anon_1_NaN as i32);
    }
    return (anon_1_non_zero as i32);
}
pub fn is_about_5(mut x: f32, mut y: f32) {
    assert!(((({ libcc2rs::abs_refcount(((x - y) / (x + y)),) }) as f64) < 1e-06_f64));
}
pub fn is_about_6(mut x: f64, mut y: f64) {
    assert!((({ libcc2rs::abs_refcount(((x - y) / (x + y)),) }) < 1e-14_f64));
}
pub fn is_about_7(mut x: f64, mut y: f64) {
    assert!((({ libcc2rs::abs_refcount(((x - y) / (x + y)),) }) < 1e-14_f64));
}
pub fn test_8(a: Ptr<f32>, b: Ptr<std_complex_float_>, x: std_complex_float_) {
    let x: Value<std_complex_float_> = Rc::new(RefCell::new(x));
    let c: Value<std_complex_float_> = Rc::new(RefCell::new(
        ({
            let ___x: Ptr<f32> = (a).clone();
            let ___y: Ptr<std_complex_float_> = (b).clone();
            pow_9(___x, ___y)
        }),
    ));
    ({
        let _x: f32 = ({ real_10(c.as_pointer()) });
        let _y: f32 = ({ real_10(x.as_pointer()) });
        is_about_5(_x, _y)
    });
    assert!(((({ libcc2rs::abs_refcount(({ imag_11(c.as_pointer(),) }),) }) as f64) < 1e-06_f64));
}
pub fn test_12(a: Ptr<f64>, b: Ptr<std_complex_double_>, x: std_complex_double_) {
    let x: Value<std_complex_double_> = Rc::new(RefCell::new(x));
    let c: Value<std_complex_double_> = Rc::new(RefCell::new(
        ({
            let ___x: Ptr<f64> = (a).clone();
            let ___y: Ptr<std_complex_double_> = (b).clone();
            pow_13(___x, ___y)
        }),
    ));
    ({
        let _x: f64 = ({ real_14(c.as_pointer()) });
        let _y: f64 = ({ real_14(x.as_pointer()) });
        is_about_6(_x, _y)
    });
    assert!((({ libcc2rs::abs_refcount(({ imag_15(c.as_pointer(),) }),) }) < 1e-06_f64));
}
pub fn test_16(a: Ptr<f64>, b: Ptr<std_complex_long_double_>, x: std_complex_long_double_) {
    let x: Value<std_complex_long_double_> = Rc::new(RefCell::new(x));
    let c: Value<std_complex_long_double_> = Rc::new(RefCell::new(
        ({
            let ___x: Ptr<f64> = (a).clone();
            let ___y: Ptr<std_complex_long_double_> = (b).clone();
            pow_17(___x, ___y)
        }),
    ));
    ({
        let _x: f64 = ({ real_18(c.as_pointer()) });
        let _y: f64 = ({ real_18(x.as_pointer()) });
        is_about_7(_x, _y)
    });
    assert!((({ libcc2rs::abs_refcount(({ imag_19(c.as_pointer(),) }),) }) < 1e-06_f64));
}
pub fn test_20() {
    ({
        let _a: Value<f32> = Rc::new(RefCell::new(2_f32));
        let _b: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
        test_8(
            _a.as_pointer(),
            _b.as_pointer(),
            <std_complex_float_>::default(),
        )
    });
}
pub fn test_21() {
    ({
        let _a: Value<f64> = Rc::new(RefCell::new(2_f64));
        let _b: Value<std_complex_double_> =
            Rc::new(RefCell::new(<std_complex_double_>::default()));
        test_12(
            _a.as_pointer(),
            _b.as_pointer(),
            <std_complex_double_>::default(),
        )
    });
}
pub fn test_22() {
    ({
        let _a: Value<f64> = Rc::new(RefCell::new(2_f64));
        let _b: Value<std_complex_long_double_> =
            Rc::new(RefCell::new(<std_complex_long_double_>::default()));
        test_16(
            _a.as_pointer(),
            _b.as_pointer(),
            <std_complex_long_double_>::default(),
        )
    });
}
pub fn test_edges_23() {
    let mut N: u32 = (((2448usize as usize).wrapping_div((16usize as usize))) as u32);
    let mut i: u32 = 0_u32;
    'loop_: while (i < N) {
        let mut j: u32 = 0_u32;
        'loop_: while (j < N) {
            let r: Value<std_complex_double_> = Rc::new(RefCell::new(
                ({
                    let ___x: Value<f64> = Rc::new(RefCell::new(
                        ({
                            real_14(
                                (testcases_0.with(|v| v.as_pointer()) as Ptr<std_complex_double_>)
                                    .offset(i),
                            )
                        }),
                    ));
                    let ___y: Ptr<std_complex_double_> = (testcases_0.with(|v| v.as_pointer())
                        as Ptr<std_complex_double_>)
                        .offset(j);
                    pow_13(___x.as_pointer(), ___y)
                }),
            ));
            let z: Value<std_complex_double_> = Rc::new(RefCell::new(
                ({
                    let ___z: Value<std_complex_double_> = Rc::new(RefCell::new(
                        (*(*testcases_0.with(Value::clone).borrow())[(j) as usize]
                            .upgrade()
                            .deref())
                        .clone(),
                    ));
                    exp_24(___z.as_pointer())
                }),
            ));
            if ({ isnan_4(({ real_14(r.as_pointer()) })) }) {
                assert!(({ isnan_4(({ real_14(z.as_pointer(),) }),) }));
            } else {
                assert!((({ real_14(r.as_pointer(),) }) == ({ real_14(z.as_pointer(),) })));
            }
            if ({ isnan_4(({ imag_15(r.as_pointer()) })) }) {
                assert!(({ isnan_4(({ imag_15(z.as_pointer(),) }),) }));
            } else {
                assert!((({ imag_15(r.as_pointer(),) }) == ({ imag_15(z.as_pointer(),) })));
            }
            j.prefix_inc();
        }
        i.prefix_inc();
    }
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
    ({ test_20() });
    ({ test_21() });
    ({ test_22() });
    ({ test_edges_23() });
    return 0;
}
pub fn __cpp2rust_init_globals() {
    let _ = testcases_0.with(|_| ());
}
