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
pub fn test_8(c: Ptr<std_complex_float_>, x: std_complex_float_) {
    let x: Value<std_complex_float_> = Rc::new(RefCell::new(x));
    let a: Value<std_complex_float_> = Rc::new(RefCell::new(({ sqrt_9((c).clone()) })));
    ({
        let _x: f32 = ({ real_10(a.as_pointer()) });
        let _y: f32 = ({ real_10(x.as_pointer()) });
        is_about_5(_x, _y)
    });
    assert!(((({ libcc2rs::abs_refcount(({ imag_11((c).clone(),) }),) }) as f64) < 1e-06_f64));
}
pub fn test_12(c: Ptr<std_complex_double_>, x: std_complex_double_) {
    let x: Value<std_complex_double_> = Rc::new(RefCell::new(x));
    let a: Value<std_complex_double_> = Rc::new(RefCell::new(({ sqrt_13((c).clone()) })));
    ({
        let _x: f64 = ({ real_14(a.as_pointer()) });
        let _y: f64 = ({ real_14(x.as_pointer()) });
        is_about_6(_x, _y)
    });
    assert!((({ libcc2rs::abs_refcount(({ imag_15((c).clone(),) }),) }) < 1e-06_f64));
}
pub fn test_16(c: Ptr<std_complex_long_double_>, x: std_complex_long_double_) {
    let x: Value<std_complex_long_double_> = Rc::new(RefCell::new(x));
    let a: Value<std_complex_long_double_> = Rc::new(RefCell::new(({ sqrt_17((c).clone()) })));
    ({
        let _x: f64 = ({ real_18(a.as_pointer()) });
        let _y: f64 = ({ real_18(x.as_pointer()) });
        is_about_7(_x, _y)
    });
    assert!((({ libcc2rs::abs_refcount(({ imag_19((c).clone(),) }),) }) < 1e-06_f64));
}
pub fn test_20() {
    ({
        let _c: Value<std_complex_float_> = Rc::new(RefCell::new(<std_complex_float_>::default()));
        test_8(_c.as_pointer(), <std_complex_float_>::default())
    });
}
pub fn test_21() {
    ({
        let _c: Value<std_complex_double_> =
            Rc::new(RefCell::new(<std_complex_double_>::default()));
        test_12(_c.as_pointer(), <std_complex_double_>::default())
    });
}
pub fn test_22() {
    ({
        let _c: Value<std_complex_long_double_> =
            Rc::new(RefCell::new(<std_complex_long_double_>::default()));
        test_16(_c.as_pointer(), <std_complex_long_double_>::default())
    });
}
pub fn test_edges_23() {
    let mut N: u32 = (((2448usize as usize).wrapping_div((16usize as usize))) as u32);
    let mut i: u32 = 0_u32;
    'loop_: while (i < N) {
        let r: Value<std_complex_double_> = Rc::new(RefCell::new(
            ({
                sqrt_13(
                    (testcases_0.with(|v| v.as_pointer()) as Ptr<std_complex_double_>).offset(i),
                )
            }),
        ));
        if (({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].real_5() }) == 0_f64)
            && (({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].imag_6() }) == 0_f64)
        {
            assert!(({ !({ signbit_24(({ (*r.borrow()).real_5() }),) }) }));
            assert!(
                ({
                    ((({ signbit_24(({ (*r.borrow()).imag_6() })) }) as i32)
                        == (({
                            signbit_24(
                                ({
                                    (*testcases_0.with(Value::clone).borrow())[(i) as usize]
                                        .imag_6()
                                }),
                            )
                        }) as i32))
                })
            );
        } else if ({
            isinf_3(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].imag_6() }))
        }) {
            assert!(({ ({ isinf_3(({ (*r.borrow()).real_5() }),) }) }));
            assert!(({ (({ (*r.borrow()).real_5() }) > 0_f64) }));
            assert!(({ ({ isinf_3(({ (*r.borrow()).imag_6() }),) }) }));
            assert!(
                ({
                    ((({ signbit_24(({ (*r.borrow()).imag_6() })) }) as i32)
                        == (({
                            signbit_24(
                                ({
                                    (*testcases_0.with(Value::clone).borrow())[(i) as usize]
                                        .imag_6()
                                }),
                            )
                        }) as i32))
                })
            );
        } else if ({
            isfinite_25(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].real_5() }))
        }) && ({
            isnan_4(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].imag_6() }))
        }) {
            assert!(({ ({ isnan_4(({ (*r.borrow()).real_5() }),) }) }));
            assert!(({ ({ isnan_4(({ (*r.borrow()).imag_6() }),) }) }));
        } else if (({
            isinf_3(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].real_5() }))
        }) && (({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].real_5() })
            < 0_f64))
            && ({
                isfinite_25(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].imag_6() }))
            })
        {
            assert!(({ (({ (*r.borrow()).real_5() }) == 0_f64) }));
            assert!(({ !({ signbit_24(({ (*r.borrow()).real_5() }),) }) }));
            assert!(({ ({ isinf_3(({ (*r.borrow()).imag_6() }),) }) }));
            assert!(
                ({
                    ((({
                        signbit_24(
                            ({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].imag_6() }),
                        )
                    }) as i32)
                        == (({ signbit_24(({ (*r.borrow()).imag_6() })) }) as i32))
                })
            );
        } else if (({
            isinf_3(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].real_5() }))
        }) && (({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].real_5() })
            > 0_f64))
            && ({
                isfinite_25(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].imag_6() }))
            })
        {
            assert!(({ ({ isinf_3(({ (*r.borrow()).real_5() }),) }) }));
            assert!(({ (({ (*r.borrow()).real_5() }) > 0_f64) }));
            assert!(({ (({ (*r.borrow()).imag_6() }) == 0_f64) }));
            assert!(
                ({
                    ((({
                        signbit_24(
                            ({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].imag_6() }),
                        )
                    }) as i32)
                        == (({ signbit_24(({ (*r.borrow()).imag_6() })) }) as i32))
                })
            );
        } else if (({
            isinf_3(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].real_5() }))
        }) && (({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].real_5() })
            < 0_f64))
            && ({
                isnan_4(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].imag_6() }))
            })
        {
            assert!(({ ({ isnan_4(({ (*r.borrow()).real_5() }),) }) }));
            assert!(({ ({ isinf_3(({ (*r.borrow()).imag_6() }),) }) }));
        } else if (({
            isinf_3(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].real_5() }))
        }) && (({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].real_5() })
            > 0_f64))
            && ({
                isnan_4(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].imag_6() }))
            })
        {
            assert!(({ ({ isinf_3(({ (*r.borrow()).real_5() }),) }) }));
            assert!(({ (({ (*r.borrow()).real_5() }) > 0_f64) }));
            assert!(({ ({ isnan_4(({ (*r.borrow()).imag_6() }),) }) }));
        } else if ({
            isnan_4(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].real_5() }))
        }) && (({
            isfinite_25(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].imag_6() }))
        }) || ({
            isnan_4(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].imag_6() }))
        })) {
            assert!(({ ({ isnan_4(({ (*r.borrow()).real_5() }),) }) }));
            assert!(({ ({ isnan_4(({ (*r.borrow()).imag_6() }),) }) }));
        } else if ({
            signbit_24(({ (*testcases_0.with(Value::clone).borrow())[(i) as usize].imag_6() }))
        }) {
            assert!(({ !({ signbit_24(({ (*r.borrow()).real_5() }),) }) }));
            assert!(({ ({ signbit_24(({ (*r.borrow()).imag_6() }),) }) }));
        } else {
            assert!(({ !({ signbit_24(({ (*r.borrow()).real_5() }),) }) }));
            assert!(({ !({ signbit_24(({ (*r.borrow()).imag_6() }),) }) }));
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
