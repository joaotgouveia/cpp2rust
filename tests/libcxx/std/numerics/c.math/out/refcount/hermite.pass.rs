extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_float__double__long_double_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_type_list_short__int__long__long_long_ {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
pub fn for_each_0(mut _a0: types_type_list_float__double__long_double_, f: TestFloat) {
    let f: Value<TestFloat> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ TestFloatImpl::operator_call_1_float(&f.as_pointer()) });
            0_()
        };
        let _arg1: () = {
            ({ TestFloatImpl::operator_call_1_double(&f.as_pointer()) });
            0_()
        };
        let _arg2: () = {
            ({ TestFloatImpl::operator_call_1_long_double(&f.as_pointer()) });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2)
    });
}
pub fn for_each_2(mut _a0: types_type_list_short__int__long__long_long_, f: TestInt) {
    let f: Value<TestInt> = Rc::new(RefCell::new(f));
    ({
        let _arg0: () = {
            ({ TestIntImpl::operator_call_1_short(&f.as_pointer()) });
            0_()
        };
        let _arg1: () = {
            ({ TestIntImpl::operator_call_1_int(&f.as_pointer()) });
            0_()
        };
        let _arg2: () = {
            ({ TestIntImpl::operator_call_1_long(&f.as_pointer()) });
            0_()
        };
        let _arg3: () = {
            ({ TestIntImpl::operator_call_1_long_long(&f.as_pointer()) });
            0_()
        };
        swallow_3(_arg0, _arg1, _arg2, _arg3)
    });
}
pub fn swallow_1(mut _a0: (), mut _a1: (), mut _a2: ()) {}
pub fn swallow_3(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: ()) {}
pub fn get_maximal_order_4() -> () {
    {
        return 128_();
    }
    panic!("ub: non-void function does not return a value")
}
pub fn get_maximal_order_5() -> () {
    {
        return 128_();
    }
    panic!("ub: non-void function does not return a value")
}
pub fn get_maximal_order_6() -> () {
    {
        return 128_();
    }
    panic!("ub: non-void function does not return a value")
}
pub fn sample_points_7() -> () {
    return unimplemented!();
}
pub fn sample_points_8() -> () {
    return unimplemented!();
}
pub fn sample_points_9() -> () {
    return unimplemented!();
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(8)]
pub struct CompareFloatingValues_float_ {
    #[offset(0)]
    abs_tol: (),
    #[offset(4)]
    rel_tol: (),
}
impl CompareFloatingValues_float_ {
    pub fn new() -> Self {
        let __this: Value<CompareFloatingValues_float_> = Rc::new(RefCell::new(Self {
            abs_tol: unimplemented!(),
            rel_tol: unimplemented!(),
        }));
        let this: Ptr<CompareFloatingValues_float_> = __this.as_pointer();
        field!(this, abs_tol).write(
            ({
                FnPtr::<fn() -> ()>::new(|| -> () {
                    {
                        {
                            return 1e-05_f32;
                        }
                        panic!("ub: non-void function does not return a value")
                    }
                })
                .call()
            }),
        );
        field!(this, rel_tol).write(this.with(|__s| __s.abs_tol));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for CompareFloatingValues_float_ {
    fn default() -> Self {
        { CompareFloatingValues_float_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(16)]
pub struct CompareFloatingValues_double_ {
    #[offset(0)]
    abs_tol: (),
    #[offset(8)]
    rel_tol: (),
}
impl CompareFloatingValues_double_ {
    pub fn new() -> Self {
        let __this: Value<CompareFloatingValues_double_> = Rc::new(RefCell::new(Self {
            abs_tol: unimplemented!(),
            rel_tol: unimplemented!(),
        }));
        let this: Ptr<CompareFloatingValues_double_> = __this.as_pointer();
        field!(this, abs_tol).write(
            ({
                FnPtr::<fn() -> ()>::new(|| -> () {
                    {
                        {
                            {
                                return 1e-11_f64;
                            }
                        }
                        panic!("ub: non-void function does not return a value")
                    }
                })
                .call()
            }),
        );
        field!(this, rel_tol).write(this.with(|__s| __s.abs_tol));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for CompareFloatingValues_double_ {
    fn default() -> Self {
        { CompareFloatingValues_double_::new() }
    }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(32)]
pub struct CompareFloatingValues_long_double_ {
    #[offset(0)]
    #[byte_size(16)]
    abs_tol: (),
    #[offset(16)]
    #[byte_size(16)]
    rel_tol: (),
}
impl CompareFloatingValues_long_double_ {
    pub fn new() -> Self {
        let __this: Value<CompareFloatingValues_long_double_> = Rc::new(RefCell::new(Self {
            abs_tol: unimplemented!(),
            rel_tol: unimplemented!(),
        }));
        let this: Ptr<CompareFloatingValues_long_double_> = __this.as_pointer();
        field!(this, abs_tol).write(
            ({
                FnPtr::<fn() -> ()>::new(|| -> () {
                    {
                        {
                            {
                                return 1e-12_f64;
                            }
                        }
                        panic!("ub: non-void function does not return a value")
                    }
                })
                .call()
            }),
        );
        field!(this, rel_tol).write(this.with(|__s| __s.abs_tol));
        Rc::try_unwrap(__this).ok().unwrap().into_inner()
    }
}
impl Default for CompareFloatingValues_long_double_ {
    fn default() -> Self {
        { CompareFloatingValues_long_double_::new() }
    }
}
pub fn get_roots_19(mut n: ()) -> () {
    'switch: {
        match { n } {
            __v if __v == 0_() => {
                return unimplemented!();
            }
            __v if __v == 1_() => {
                return unimplemented!();
            }
            __v if __v == 2_() => {
                return unimplemented!();
            }
            __v if __v == 3_() => {
                return unimplemented!();
            }
            __v if __v == 4_() => {
                return unimplemented!();
            }
            __v if __v == 5_() => {
                return unimplemented!();
            }
            __v if __v == 6_() => {
                return unimplemented!();
            }
            __v if __v == 7_() => {
                return unimplemented!();
            }
            __v if __v == 8_() => {
                return unimplemented!();
            }
            __v if __v == 9_() => {
                return unimplemented!();
            }
            __v if __v == 10_() => {
                return unimplemented!();
            }
            __v if __v == 11_() => {
                return unimplemented!();
            }
            __v if __v == 12_() => {
                return unimplemented!();
            }
            __v if __v == 13_() => {
                return unimplemented!();
            }
            __v if __v == 14_() => {
                return unimplemented!();
            }
            __v if __v == 15_() => {
                return unimplemented!();
            }
            __v if __v == 16_() => {
                return unimplemented!();
            }
            __v if __v == 17_() => {
                return unimplemented!();
            }
            __v if __v == 18_() => {
                return unimplemented!();
            }
            __v if __v == 19_() => {
                return unimplemented!();
            }
            __v if __v == 20_() => {
                return unimplemented!();
            }
            _ => {
                unimplemented!();
                return unimplemented!();
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn get_roots_20(mut n: ()) -> () {
    'switch: {
        match { n } {
            __v if __v == 0_() => {
                return unimplemented!();
            }
            __v if __v == 1_() => {
                return unimplemented!();
            }
            __v if __v == 2_() => {
                return unimplemented!();
            }
            __v if __v == 3_() => {
                return unimplemented!();
            }
            __v if __v == 4_() => {
                return unimplemented!();
            }
            __v if __v == 5_() => {
                return unimplemented!();
            }
            __v if __v == 6_() => {
                return unimplemented!();
            }
            __v if __v == 7_() => {
                return unimplemented!();
            }
            __v if __v == 8_() => {
                return unimplemented!();
            }
            __v if __v == 9_() => {
                return unimplemented!();
            }
            __v if __v == 10_() => {
                return unimplemented!();
            }
            __v if __v == 11_() => {
                return unimplemented!();
            }
            __v if __v == 12_() => {
                return unimplemented!();
            }
            __v if __v == 13_() => {
                return unimplemented!();
            }
            __v if __v == 14_() => {
                return unimplemented!();
            }
            __v if __v == 15_() => {
                return unimplemented!();
            }
            __v if __v == 16_() => {
                return unimplemented!();
            }
            __v if __v == 17_() => {
                return unimplemented!();
            }
            __v if __v == 18_() => {
                return unimplemented!();
            }
            __v if __v == 19_() => {
                return unimplemented!();
            }
            __v if __v == 20_() => {
                return unimplemented!();
            }
            _ => {
                unimplemented!();
                return unimplemented!();
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn get_roots_21(mut n: ()) -> () {
    'switch: {
        match { n } {
            __v if __v == 0_() => {
                return unimplemented!();
            }
            __v if __v == 1_() => {
                return unimplemented!();
            }
            __v if __v == 2_() => {
                return unimplemented!();
            }
            __v if __v == 3_() => {
                return unimplemented!();
            }
            __v if __v == 4_() => {
                return unimplemented!();
            }
            __v if __v == 5_() => {
                return unimplemented!();
            }
            __v if __v == 6_() => {
                return unimplemented!();
            }
            __v if __v == 7_() => {
                return unimplemented!();
            }
            __v if __v == 8_() => {
                return unimplemented!();
            }
            __v if __v == 9_() => {
                return unimplemented!();
            }
            __v if __v == 10_() => {
                return unimplemented!();
            }
            __v if __v == 11_() => {
                return unimplemented!();
            }
            __v if __v == 12_() => {
                return unimplemented!();
            }
            __v if __v == 13_() => {
                return unimplemented!();
            }
            __v if __v == 14_() => {
                return unimplemented!();
            }
            __v if __v == 15_() => {
                return unimplemented!();
            }
            __v if __v == 16_() => {
                return unimplemented!();
            }
            __v if __v == 17_() => {
                return unimplemented!();
            }
            __v if __v == 18_() => {
                return unimplemented!();
            }
            __v if __v == 19_() => {
                return unimplemented!();
            }
            __v if __v == 20_() => {
                return unimplemented!();
            }
            _ => {
                unimplemented!();
                return unimplemented!();
            }
        }
    };
    panic!("ub: non-void function does not return a value")
}
pub fn test_22() {
    {
        let __tmp_0: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let __range2: Ptr<()> = __tmp_0.as_pointer();
        'loop_: for NaN in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut NaN: () = (NaN.read());
            let mut n: () = 0_();
            'loop_: while (n < ({ get_maximal_order_5() })) {
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let __tmp_1: Value<()> = Rc::new(RefCell::new(({ sample_points_7() })));
        let __range2: Ptr<()> = __tmp_1.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (x.read());
            let mut n: () = 0_();
            'loop_: while (n < ({ get_maximal_order_5() })) {
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let h0: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut _a0: ()| -> () {
                {
                    return 1_();
                }
            },
        )));
        let h1: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (2_() * y);
                }
            },
        )));
        let h2: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (((4_() * y) * y) - 2_());
                }
            },
        )));
        let h3: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (y * (((8_() * y) * y) - 12_()));
                }
            },
        )));
        let h4: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (((16_() * ({ pow_23(y, 4_()) })) - ((48_() * y) * y)) + 12_());
                }
            },
        )));
        let h5: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (y * (((32_() * ({ pow_23(y, 4_()) })) - ((160_() * y) * y)) + 120_()));
                }
            },
        )));
        let __tmp_2: Value<()> = Rc::new(RefCell::new(({ sample_points_7() })));
        let __range2: Ptr<()> = __tmp_2.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (x.read());
            let compare: Value<CompareFloatingValues_float_> =
                Rc::new(RefCell::new(CompareFloatingValues_float_::new()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {
            let mut n: () = 0_();
            'loop_: while (n < ({ get_maximal_order_5() })) {
                let __tmp_3: Value<()> = Rc::new(RefCell::new(({ sample_points_7() })));
                let __range4: Ptr<()> = __tmp_3.as_pointer();
                'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                    let mut x: () = (x.read());
                    unimplemented!();
                }
                n.prefix_inc();
            }
        }
    }
    {
        {}
    }
    {
        let __tmp_4: Value<()> = Rc::new(RefCell::new(({ sample_points_7() })));
        let __range2: Ptr<()> = __tmp_4.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (x.read());
            let mut n: () = 1_();
            'loop_: while (n < (({ get_maximal_order_5() }) as ()).wrapping_sub(1_())) {
                let mut H_next: () = unimplemented!();
                let mut H_next_recurrence: () =
                    (2_() * ((x * unimplemented!()) - (n * unimplemented!())));
                if ({ isinf_10(H_next) }) {
                    break;
                }
                let compare: Value<CompareFloatingValues_float_> =
                    Rc::new(RefCell::new(CompareFloatingValues_float_::new()));
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let tol: Value<()> = Rc::new(RefCell::new(
            ({
                FnPtr::<fn() -> ()>::new(|| -> () {
                    {
                        {
                            return 1e-05_f32;
                        }
                        panic!("ub: non-void function does not return a value")
                    }
                })
                .call()
            }),
        ));
        let is_sign_change: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(lambda!(
            {
                let tol: Value<()> = Rc::new(RefCell::new((*tol.borrow())));
            },
            |mut n: (), mut x: ()| -> () {
                return ((unimplemented!() * unimplemented!()) < 0_());
            }
        )));
        let mut n: () = 0_();
        'loop_: while (n <= 20_()) {
            let __tmp_5: Value<()> = Rc::new(RefCell::new(({ get_roots_19(n) })));
            let __range3: Ptr<()> = __tmp_5.as_pointer();
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (unimplemented!().read());
                if (x > 0_()) {
                    unimplemented!();
                }
                unimplemented!();
            }
            n.prefix_inc();
        }
    }
    {
        let mut inf: () = unimplemented!();
        let mut n: () = 1_();
        'loop_: while (n < ({ get_maximal_order_5() })) {
            unimplemented!();
            unimplemented!();
            n.prefix_inc();
        }
    }
    { {} }
}
pub fn test_24() {
    {
        let __tmp_6: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let __range2: Ptr<()> = __tmp_6.as_pointer();
        'loop_: for NaN in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut NaN: () = (NaN.read());
            let mut n: () = 0_();
            'loop_: while (n < ({ get_maximal_order_4() })) {
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let __tmp_7: Value<()> = Rc::new(RefCell::new(({ sample_points_9() })));
        let __range2: Ptr<()> = __tmp_7.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (x.read());
            let mut n: () = 0_();
            'loop_: while (n < ({ get_maximal_order_4() })) {
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let h0: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut _a0: ()| -> () {
                {
                    return 1_();
                }
            },
        )));
        let h1: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (2_() * y);
                }
            },
        )));
        let h2: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (((4_() * y) * y) - 2_());
                }
            },
        )));
        let h3: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (y * (((8_() * y) * y) - 12_()));
                }
            },
        )));
        let h4: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (((16_() * ({ pow_25(y, 4_()) })) - ((48_() * y) * y)) + 12_());
                }
            },
        )));
        let h5: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (y * (((32_() * ({ pow_25(y, 4_()) })) - ((160_() * y) * y)) + 120_()));
                }
            },
        )));
        let __tmp_8: Value<()> = Rc::new(RefCell::new(({ sample_points_9() })));
        let __range2: Ptr<()> = __tmp_8.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (x.read());
            let compare: Value<CompareFloatingValues_double_> =
                Rc::new(RefCell::new(CompareFloatingValues_double_::new()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {}
    }
    {
        {}
    }
    {
        let __tmp_9: Value<()> = Rc::new(RefCell::new(({ sample_points_9() })));
        let __range2: Ptr<()> = __tmp_9.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (x.read());
            let mut n: () = 1_();
            'loop_: while (n < (({ get_maximal_order_4() }) as ()).wrapping_sub(1_())) {
                let mut H_next: () = unimplemented!();
                let mut H_next_recurrence: () =
                    (2_() * ((x * unimplemented!()) - (n * unimplemented!())));
                if ({ isinf_13(H_next) }) {
                    break;
                }
                let compare: Value<CompareFloatingValues_double_> =
                    Rc::new(RefCell::new(CompareFloatingValues_double_::new()));
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let tol: Value<()> = Rc::new(RefCell::new(
            ({
                FnPtr::<fn() -> ()>::new(|| -> () {
                    {
                        {
                            {
                                return 1e-09_f64;
                            }
                        }
                        panic!("ub: non-void function does not return a value")
                    }
                })
                .call()
            }),
        ));
        let is_sign_change: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(lambda!(
            {
                let tol: Value<()> = Rc::new(RefCell::new((*tol.borrow())));
            },
            |mut n: (), mut x: ()| -> () {
                return ((unimplemented!() * unimplemented!()) < 0_());
            }
        )));
        let mut n: () = 0_();
        'loop_: while (n <= 20_()) {
            let __tmp_10: Value<()> = Rc::new(RefCell::new(({ get_roots_20(n) })));
            let __range3: Ptr<()> = __tmp_10.as_pointer();
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (unimplemented!().read());
                if (x > 0_()) {
                    unimplemented!();
                }
                unimplemented!();
            }
            n.prefix_inc();
        }
    }
    {
        let mut inf: () = unimplemented!();
        let mut n: () = 1_();
        'loop_: while (n < ({ get_maximal_order_4() })) {
            unimplemented!();
            unimplemented!();
            n.prefix_inc();
        }
    }
    {
        {
            const _: () = assert!((::std::mem::size_of::<()>() == 8_()), "sizeof(double) == 8");;
            let mut n: () = 0_();
            'loop_: while (n < ({ get_maximal_order_4() })) {
                if (n < 111_()) {
                    unimplemented!();
                    unimplemented!();
                } else {
                    let mut inf: () = unimplemented!();
                    unimplemented!();
                    unimplemented!();
                }
                n.prefix_inc();
            }
        }
    }
}
pub fn test_26() {
    {
        let __tmp_11: Value<()> = Rc::new(RefCell::new(unimplemented!()));
        let __range2: Ptr<()> = __tmp_11.as_pointer();
        'loop_: for NaN in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut NaN: () = (NaN.read());
            let mut n: () = 0_();
            'loop_: while (n < ({ get_maximal_order_6() })) {
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let __tmp_12: Value<()> = Rc::new(RefCell::new(({ sample_points_8() })));
        let __range2: Ptr<()> = __tmp_12.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (x.read());
            let mut n: () = 0_();
            'loop_: while (n < ({ get_maximal_order_6() })) {
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let h0: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut _a0: ()| -> () {
                {
                    return 1_();
                }
            },
        )));
        let h1: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (2_() * y);
                }
            },
        )));
        let h2: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (((4_() * y) * y) - 2_());
                }
            },
        )));
        let h3: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (y * (((8_() * y) * y) - 12_()));
                }
            },
        )));
        let h4: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (((16_() * ({ pow_27(y, 4_()) })) - ((48_() * y) * y)) + 12_());
                }
            },
        )));
        let h5: Value<FnPtr<fn(()) -> ()>> = Rc::new(RefCell::new(FnPtr::<fn(()) -> ()>::new(
            |mut y: ()| -> () {
                {
                    return (y * (((32_() * ({ pow_27(y, 4_()) })) - ((160_() * y) * y)) + 120_()));
                }
            },
        )));
        let __tmp_13: Value<()> = Rc::new(RefCell::new(({ sample_points_8() })));
        let __range2: Ptr<()> = __tmp_13.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (x.read());
            let compare: Value<CompareFloatingValues_long_double_> =
                Rc::new(RefCell::new(CompareFloatingValues_long_double_::new()));
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        {}
    }
    {
        {
            let mut n: () = 0_();
            'loop_: while (n < ({ get_maximal_order_6() })) {
                let __tmp_14: Value<()> = Rc::new(RefCell::new(({ sample_points_8() })));
                let __range4: Ptr<()> = __tmp_14.as_pointer();
                'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                    let mut x: () = (x.read());
                    unimplemented!();
                }
                n.prefix_inc();
            }
        }
    }
    {
        let __tmp_15: Value<()> = Rc::new(RefCell::new(({ sample_points_8() })));
        let __range2: Ptr<()> = __tmp_15.as_pointer();
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (x.read());
            let mut n: () = 1_();
            'loop_: while (n < (({ get_maximal_order_6() }) as ()).wrapping_sub(1_())) {
                let mut H_next: () = unimplemented!();
                let mut H_next_recurrence: () =
                    (2_() * ((x * unimplemented!()) - (n * unimplemented!())));
                if ({ isinf_16(H_next) }) {
                    break;
                }
                let compare: Value<CompareFloatingValues_long_double_> =
                    Rc::new(RefCell::new(CompareFloatingValues_long_double_::new()));
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let tol: Value<()> = Rc::new(RefCell::new(
            ({
                FnPtr::<fn() -> ()>::new(|| -> () {
                    {
                        {
                            {
                                return 1e-10_f64;
                            }
                        }
                        panic!("ub: non-void function does not return a value")
                    }
                })
                .call()
            }),
        ));
        let is_sign_change: Value<FnPtr<fn((), ()) -> ()>> = Rc::new(RefCell::new(lambda!(
            {
                let tol: Value<()> = Rc::new(RefCell::new((*tol.borrow())));
            },
            |mut n: (), mut x: ()| -> () {
                return ((unimplemented!() * unimplemented!()) < 0_());
            }
        )));
        let mut n: () = 0_();
        'loop_: while (n <= 20_()) {
            let __tmp_16: Value<()> = Rc::new(RefCell::new(({ get_roots_21(n) })));
            let __range3: Ptr<()> = __tmp_16.as_pointer();
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (unimplemented!().read());
                if (x > 0_()) {
                    unimplemented!();
                }
                unimplemented!();
            }
            n.prefix_inc();
        }
    }
    {
        let mut inf: () = unimplemented!();
        let mut n: () = 1_();
        'loop_: while (n < ({ get_maximal_order_6() })) {
            unimplemented!();
            unimplemented!();
            n.prefix_inc();
        }
    }
    { {} }
}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestFloat {}
#[derive(Clone, Record, ByteRepr, VaArg, FnPtrArg, Default)]
#[byte_size(1)]
pub struct TestInt {}
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
    ({
        for_each_0(
            <types_type_list_float__double__long_double_>::default(),
            <TestFloat>::default(),
        )
    });
    ({
        for_each_2(
            <types_type_list_short__int__long__long_long_>::default(),
            <TestInt>::default(),
        )
    });
    return 0_();
}
pub trait CompareFloatingValues_double_Impl {
    fn operator_call(&self, result: (), expected: ()) -> ();
}
impl CompareFloatingValues_double_Impl for Ptr<CompareFloatingValues_double_> {
    fn operator_call(&self, mut result: (), mut expected: ()) -> () {
        if ({ isinf_13(expected) }) && ({ isinf_13(result) }) {
            return (result == expected);
        }
        if ({ isnan_14(expected) }) || ({ isnan_14(result) }) {
            return false;
        }
        let mut tol: () = ((*self).with(|__s| __s.abs_tol)
            + (({ abs_15(expected) }) * (*self).with(|__s| __s.rel_tol)));
        return (({ abs_15((result - expected)) }) < tol);
    }
}
pub trait CompareFloatingValues_float_Impl {
    fn operator_call(&self, result: (), expected: ()) -> ();
}
impl CompareFloatingValues_float_Impl for Ptr<CompareFloatingValues_float_> {
    fn operator_call(&self, mut result: (), mut expected: ()) -> () {
        if ({ isinf_10(expected) }) && ({ isinf_10(result) }) {
            return (result == expected);
        }
        if ({ isnan_11(expected) }) || ({ isnan_11(result) }) {
            return false;
        }
        let mut tol: () = ((*self).with(|__s| __s.abs_tol)
            + (({ abs_12(expected) }) * (*self).with(|__s| __s.rel_tol)));
        return (({ abs_12((result - expected)) }) < tol);
    }
}
pub trait CompareFloatingValues_long_double_Impl {
    fn operator_call(&self, result: (), expected: ()) -> ();
}
impl CompareFloatingValues_long_double_Impl for Ptr<CompareFloatingValues_long_double_> {
    fn operator_call(&self, mut result: (), mut expected: ()) -> () {
        if ({ isinf_16(expected) }) && ({ isinf_16(result) }) {
            return (result == expected);
        }
        if ({ isnan_17(expected) }) || ({ isnan_17(result) }) {
            return false;
        }
        let mut tol: () = ((*self).with(|__s| __s.abs_tol)
            + (({ abs_18(expected) }) * (*self).with(|__s| __s.rel_tol)));
        return (({ abs_18((result - expected)) }) < tol);
    }
}
pub trait TestFloatImpl {
    fn operator_call_1_float(&self);
    fn operator_call_1_double(&self);
    fn operator_call_1_long_double(&self);
}
impl TestFloatImpl for Ptr<TestFloat> {
    fn operator_call_1_float(&self) {
        ({ test_22() });
    }
    fn operator_call_1_double(&self) {
        ({ test_24() });
    }
    fn operator_call_1_long_double(&self) {
        ({ test_26() });
    }
}
pub trait TestIntImpl {
    fn operator_call_1_short(&self);
    fn operator_call_1_int(&self);
    fn operator_call_1_long(&self);
    fn operator_call_1_long_long(&self);
}
impl TestIntImpl for Ptr<TestInt> {
    fn operator_call_1_short(&self) {
        let mut n: () = 0_();
        'loop_: while (n < ({ get_maximal_order_4() })) {
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (x.read());
                unimplemented!();
            }
            n.prefix_inc();
        }
    }
    fn operator_call_1_int(&self) {
        let mut n: () = 0_();
        'loop_: while (n < ({ get_maximal_order_4() })) {
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (x.read());
                unimplemented!();
            }
            n.prefix_inc();
        }
    }
    fn operator_call_1_long(&self) {
        let mut n: () = 0_();
        'loop_: while (n < ({ get_maximal_order_4() })) {
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (x.read());
                unimplemented!();
            }
            n.prefix_inc();
        }
    }
    fn operator_call_1_long_long(&self) {
        let mut n: () = 0_();
        'loop_: while (n < ({ get_maximal_order_4() })) {
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (x.read());
                unimplemented!();
            }
            n.prefix_inc();
        }
    }
}
pub fn __cpp2rust_init_globals() {}
