extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_float__double__long_double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_type_list_short__int__long__long_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t___types_type_list_signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128___types_type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128___types_type_list_bool__
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct types_concatenate_types_type_list_char__wchar_t__char8_t__char16_t__char32_t__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__bool___types_type_list_float__double__long_double__
{}
pub unsafe fn for_each_0(mut _a0: types_type_list_float__double__long_double_, mut f: TestFloat) {
    (unsafe {
        let _arg0: () = {
            (unsafe { TestFloat::operator_call_1_float(&mut f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { TestFloat::operator_call_1_double(&mut f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { TestFloat::operator_call_1_long_double(&mut f) });
            0_()
        };
        swallow_1(_arg0, _arg1, _arg2)
    });
}
pub unsafe fn for_each_2(mut _a0: types_type_list_short__int__long__long_long_, mut f: TestInt) {
    (unsafe {
        let _arg0: () = {
            (unsafe { TestInt::operator_call_1_short(&mut f) });
            0_()
        };
        let _arg1: () = {
            (unsafe { TestInt::operator_call_1_int(&mut f) });
            0_()
        };
        let _arg2: () = {
            (unsafe { TestInt::operator_call_1_long(&mut f) });
            0_()
        };
        let _arg3: () = {
            (unsafe { TestInt::operator_call_1_long_long(&mut f) });
            0_()
        };
        swallow_3(_arg0, _arg1, _arg2, _arg3)
    });
}
pub unsafe fn swallow_1(mut _a0: (), mut _a1: (), mut _a2: ()) {}
pub unsafe fn swallow_3(mut _a0: (), mut _a1: (), mut _a2: (), mut _a3: ()) {}
pub unsafe fn get_maximal_order_4() -> () {
    {
        return 128_();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn get_maximal_order_5() -> () {
    {
        return 128_();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn get_maximal_order_6() -> () {
    {
        return 128_();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn sample_points_7() -> () {
    return unimplemented!();
}
pub unsafe fn sample_points_8() -> () {
    return unimplemented!();
}
pub unsafe fn sample_points_9() -> () {
    return unimplemented!();
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CompareFloatingValues_float_ {
    abs_tol: (),
    rel_tol: (),
}
impl CompareFloatingValues_float_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            abs_tol: unimplemented!(),
            rel_tol: unimplemented!(),
        };
        this.abs_tol = (unsafe {
            FnPtr::<fn() -> ()>::new(|| -> () {
                unsafe {
                    {
                        return 1e-05_f32;
                    }
                    panic!("ub: non-void function does not return a value")
                }
            })
            .call()
        });
        this.rel_tol = this.abs_tol;
        this
    }
    pub unsafe fn operator_call(&self, mut result: (), mut expected: ()) -> () {
        if (unimplemented!()) && (unimplemented!()) {
            return ((result) == (expected));
        }
        if (unimplemented!()) || (unimplemented!()) {
            return false;
        }
        let mut tol: () = ((self.abs_tol) + ((unimplemented!()) * (self.rel_tol)));
        return ((unimplemented!()) < (tol));
    }
}
impl Default for CompareFloatingValues_float_ {
    fn default() -> Self {
        unsafe { CompareFloatingValues_float_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CompareFloatingValues_double_ {
    abs_tol: (),
    rel_tol: (),
}
impl CompareFloatingValues_double_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            abs_tol: unimplemented!(),
            rel_tol: unimplemented!(),
        };
        this.abs_tol = (unsafe {
            FnPtr::<fn() -> ()>::new(|| -> () {
                unsafe {
                    {
                        {
                            return 1e-11_f64;
                        }
                    }
                    panic!("ub: non-void function does not return a value")
                }
            })
            .call()
        });
        this.rel_tol = this.abs_tol;
        this
    }
    pub unsafe fn operator_call(&self, mut result: (), mut expected: ()) -> () {
        if (unimplemented!()) && (unimplemented!()) {
            return ((result) == (expected));
        }
        if (unimplemented!()) || (unimplemented!()) {
            return false;
        }
        let mut tol: () = ((self.abs_tol) + ((unimplemented!()) * (self.rel_tol)));
        return ((unimplemented!()) < (tol));
    }
}
impl Default for CompareFloatingValues_double_ {
    fn default() -> Self {
        unsafe { CompareFloatingValues_double_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct CompareFloatingValues_long_double_ {
    abs_tol: (),
    rel_tol: (),
}
impl CompareFloatingValues_long_double_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            abs_tol: unimplemented!(),
            rel_tol: unimplemented!(),
        };
        this.abs_tol = (unsafe {
            FnPtr::<fn() -> ()>::new(|| -> () {
                unsafe {
                    {
                        {
                            return 1e-12_f64;
                        }
                    }
                    panic!("ub: non-void function does not return a value")
                }
            })
            .call()
        });
        this.rel_tol = this.abs_tol;
        this
    }
    pub unsafe fn operator_call(&self, mut result: (), mut expected: ()) -> () {
        if (unimplemented!()) && (unimplemented!()) {
            return ((result) == (expected));
        }
        if (unimplemented!()) || (unimplemented!()) {
            return false;
        }
        let mut tol: () = ((self.abs_tol) + ((unimplemented!()) * (self.rel_tol)));
        return ((unimplemented!()) < (tol));
    }
}
impl Default for CompareFloatingValues_long_double_ {
    fn default() -> Self {
        unsafe { CompareFloatingValues_long_double_::new() }
    }
}
pub unsafe fn get_roots_10(mut n: ()) -> () {
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
pub unsafe fn get_roots_11(mut n: ()) -> () {
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
pub unsafe fn get_roots_12(mut n: ()) -> () {
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
pub unsafe fn test_13() {
    {
        let mut __tmp_0: () = unimplemented!();
        let __range2: *mut () = &mut __tmp_0;
        'loop_: for NaN in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut NaN: () = (*NaN);
            let mut n: () = 0_();
            'loop_: while ((n) < (unsafe { get_maximal_order_5() })) {
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let mut __tmp_1: () = (unsafe { sample_points_7() });
        let __range2: *mut () = &mut __tmp_1;
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (*x);
            let mut n: () = 0_();
            'loop_: while ((n) < (unsafe { get_maximal_order_5() })) {
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let h0: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut _a0: ()| -> () {
            unsafe {
                return 1_();
            }
        });
        let h1: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((2_()) * (y));
            }
        });
        let h2: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((((4_()) * (y)) * (y)) - (2_()));
            }
        });
        let h3: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((y) * ((((8_()) * (y)) * (y)) - (12_())));
            }
        });
        let h4: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((((16_()) * (unimplemented!())) - (((48_()) * (y)) * (y))) + (12_()));
            }
        });
        let h5: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((y)
                    * ((((32_()) * (unimplemented!())) - (((160_()) * (y)) * (y))) + (120_())));
            }
        });
        let mut __tmp_2: () = (unsafe { sample_points_7() });
        let __range2: *mut () = &mut __tmp_2;
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (*x);
            let compare: CompareFloatingValues_float_ = CompareFloatingValues_float_::new();
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
            'loop_: while ((n) < (unsafe { get_maximal_order_5() })) {
                let mut __tmp_3: () = (unsafe { sample_points_7() });
                let __range4: *mut () = &mut __tmp_3;
                'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                    let mut x: () = (*x);
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
        let mut __tmp_4: () = (unsafe { sample_points_7() });
        let __range2: *mut () = &mut __tmp_4;
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (*x);
            let mut n: () = 1_();
            'loop_: while ((n) < (((unsafe { get_maximal_order_5() }) as ()).wrapping_sub(1_()))) {
                let mut H_next: () = unimplemented!();
                let mut H_next_recurrence: () =
                    ((2_()) * (((x) * (unimplemented!())) - ((n) * (unimplemented!()))));
                if unimplemented!() {
                    break;
                }
                let compare: CompareFloatingValues_float_ = CompareFloatingValues_float_::new();
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let mut tol: () = (unsafe {
            FnPtr::<fn() -> ()>::new(|| -> () {
                unsafe {
                    {
                        return 1e-05_f32;
                    }
                    panic!("ub: non-void function does not return a value")
                }
            })
            .call()
        });
        let is_sign_change: FnPtr<fn((), ()) -> ()> = lambda_unsafe!(
            {
                let tol: () = tol;
            },
            |mut n: (), mut x: ()| -> () {
                return (((unimplemented!()) * (unimplemented!())) < (0_()));
            }
        );
        let mut n: () = 0_();
        'loop_: while ((n) <= (20_())) {
            let mut __tmp_5: () = (unsafe { get_roots_10(n) });
            let __range3: *mut () = &mut __tmp_5;
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (*unimplemented!());
                if ((x) > (0_())) {
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
        'loop_: while ((n) < (unsafe { get_maximal_order_5() })) {
            unimplemented!();
            unimplemented!();
            n.prefix_inc();
        }
    }
    { {} }
}
pub unsafe fn test_14() {
    {
        let mut __tmp_6: () = unimplemented!();
        let __range2: *mut () = &mut __tmp_6;
        'loop_: for NaN in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut NaN: () = (*NaN);
            let mut n: () = 0_();
            'loop_: while ((n) < (unsafe { get_maximal_order_4() })) {
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let mut __tmp_7: () = (unsafe { sample_points_9() });
        let __range2: *mut () = &mut __tmp_7;
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (*x);
            let mut n: () = 0_();
            'loop_: while ((n) < (unsafe { get_maximal_order_4() })) {
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let h0: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut _a0: ()| -> () {
            unsafe {
                return 1_();
            }
        });
        let h1: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((2_()) * (y));
            }
        });
        let h2: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((((4_()) * (y)) * (y)) - (2_()));
            }
        });
        let h3: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((y) * ((((8_()) * (y)) * (y)) - (12_())));
            }
        });
        let h4: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((((16_()) * (unimplemented!())) - (((48_()) * (y)) * (y))) + (12_()));
            }
        });
        let h5: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((y)
                    * ((((32_()) * (unimplemented!())) - (((160_()) * (y)) * (y))) + (120_())));
            }
        });
        let mut __tmp_8: () = (unsafe { sample_points_9() });
        let __range2: *mut () = &mut __tmp_8;
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (*x);
            let compare: CompareFloatingValues_double_ = CompareFloatingValues_double_::new();
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
        let mut __tmp_9: () = (unsafe { sample_points_9() });
        let __range2: *mut () = &mut __tmp_9;
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (*x);
            let mut n: () = 1_();
            'loop_: while ((n) < (((unsafe { get_maximal_order_4() }) as ()).wrapping_sub(1_()))) {
                let mut H_next: () = unimplemented!();
                let mut H_next_recurrence: () =
                    ((2_()) * (((x) * (unimplemented!())) - ((n) * (unimplemented!()))));
                if unimplemented!() {
                    break;
                }
                let compare: CompareFloatingValues_double_ = CompareFloatingValues_double_::new();
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let mut tol: () = (unsafe {
            FnPtr::<fn() -> ()>::new(|| -> () {
                unsafe {
                    {
                        {
                            return 1e-09_f64;
                        }
                    }
                    panic!("ub: non-void function does not return a value")
                }
            })
            .call()
        });
        let is_sign_change: FnPtr<fn((), ()) -> ()> = lambda_unsafe!(
            {
                let tol: () = tol;
            },
            |mut n: (), mut x: ()| -> () {
                return (((unimplemented!()) * (unimplemented!())) < (0_()));
            }
        );
        let mut n: () = 0_();
        'loop_: while ((n) <= (20_())) {
            let mut __tmp_10: () = (unsafe { get_roots_11(n) });
            let __range3: *mut () = &mut __tmp_10;
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (*unimplemented!());
                if ((x) > (0_())) {
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
        'loop_: while ((n) < (unsafe { get_maximal_order_4() })) {
            unimplemented!();
            unimplemented!();
            n.prefix_inc();
        }
    }
    {
        {
            const _: () = assert!(
                ((::std::mem::size_of::<()>()) == (8_())),
                "sizeof(double) == 8"
            );;
            let mut n: () = 0_();
            'loop_: while ((n) < (unsafe { get_maximal_order_4() })) {
                if ((n) < (111_())) {
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
pub unsafe fn test_15() {
    {
        let mut __tmp_11: () = unimplemented!();
        let __range2: *mut () = &mut __tmp_11;
        'loop_: for NaN in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut NaN: () = (*NaN);
            let mut n: () = 0_();
            'loop_: while ((n) < (unsafe { get_maximal_order_6() })) {
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let mut __tmp_12: () = (unsafe { sample_points_8() });
        let __range2: *mut () = &mut __tmp_12;
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (*x);
            let mut n: () = 0_();
            'loop_: while ((n) < (unsafe { get_maximal_order_6() })) {
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let h0: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut _a0: ()| -> () {
            unsafe {
                return 1_();
            }
        });
        let h1: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((2_()) * (y));
            }
        });
        let h2: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((((4_()) * (y)) * (y)) - (2_()));
            }
        });
        let h3: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((y) * ((((8_()) * (y)) * (y)) - (12_())));
            }
        });
        let h4: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((((16_()) * (unimplemented!())) - (((48_()) * (y)) * (y))) + (12_()));
            }
        });
        let h5: FnPtr<fn(()) -> ()> = FnPtr::<fn(()) -> ()>::new(|mut y: ()| -> () {
            unsafe {
                return ((y)
                    * ((((32_()) * (unimplemented!())) - (((160_()) * (y)) * (y))) + (120_())));
            }
        });
        let mut __tmp_13: () = (unsafe { sample_points_8() });
        let __range2: *mut () = &mut __tmp_13;
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (*x);
            let compare: CompareFloatingValues_long_double_ =
                CompareFloatingValues_long_double_::new();
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
            'loop_: while ((n) < (unsafe { get_maximal_order_6() })) {
                let mut __tmp_14: () = (unsafe { sample_points_8() });
                let __range4: *mut () = &mut __tmp_14;
                'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                    let mut x: () = (*x);
                    unimplemented!();
                }
                n.prefix_inc();
            }
        }
    }
    {
        let mut __tmp_15: () = (unsafe { sample_points_8() });
        let __range2: *mut () = &mut __tmp_15;
        'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut x: () = (*x);
            let mut n: () = 1_();
            'loop_: while ((n) < (((unsafe { get_maximal_order_6() }) as ()).wrapping_sub(1_()))) {
                let mut H_next: () = unimplemented!();
                let mut H_next_recurrence: () =
                    ((2_()) * (((x) * (unimplemented!())) - ((n) * (unimplemented!()))));
                if unimplemented!() {
                    break;
                }
                let compare: CompareFloatingValues_long_double_ =
                    CompareFloatingValues_long_double_::new();
                unimplemented!();
                n.prefix_inc();
            }
        }
    }
    {
        let mut tol: () = (unsafe {
            FnPtr::<fn() -> ()>::new(|| -> () {
                unsafe {
                    {
                        {
                            return 1e-10_f64;
                        }
                    }
                    panic!("ub: non-void function does not return a value")
                }
            })
            .call()
        });
        let is_sign_change: FnPtr<fn((), ()) -> ()> = lambda_unsafe!(
            {
                let tol: () = tol;
            },
            |mut n: (), mut x: ()| -> () {
                return (((unimplemented!()) * (unimplemented!())) < (0_()));
            }
        );
        let mut n: () = 0_();
        'loop_: while ((n) <= (20_())) {
            let mut __tmp_16: () = (unsafe { get_roots_12(n) });
            let __range3: *mut () = &mut __tmp_16;
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (*unimplemented!());
                if ((x) > (0_())) {
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
        'loop_: while ((n) < (unsafe { get_maximal_order_6() })) {
            unimplemented!();
            unimplemented!();
            n.prefix_inc();
        }
    }
    { {} }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestFloat {}
impl TestFloat {
    pub unsafe fn operator_call_1_float(&mut self) {
        (unsafe { test_13() });
    }
    pub unsafe fn operator_call_1_double(&mut self) {
        (unsafe { test_14() });
    }
    pub unsafe fn operator_call_1_long_double(&mut self) {
        (unsafe { test_15() });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct TestInt {}
impl TestInt {
    pub unsafe fn operator_call_1_short(&mut self) {
        let mut n: () = 0_();
        'loop_: while ((n) < (unsafe { get_maximal_order_4() })) {
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (*x);
                unimplemented!();
            }
            n.prefix_inc();
        }
    }
    pub unsafe fn operator_call_1_int(&mut self) {
        let mut n: () = 0_();
        'loop_: while ((n) < (unsafe { get_maximal_order_4() })) {
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (*x);
                unimplemented!();
            }
            n.prefix_inc();
        }
    }
    pub unsafe fn operator_call_1_long(&mut self) {
        let mut n: () = 0_();
        'loop_: while ((n) < (unsafe { get_maximal_order_4() })) {
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (*x);
                unimplemented!();
            }
            n.prefix_inc();
        }
    }
    pub unsafe fn operator_call_1_long_long(&mut self) {
        let mut n: () = 0_();
        'loop_: while ((n) < (unsafe { get_maximal_order_4() })) {
            'loop_: for x in IterRange::new(unimplemented!(), unimplemented!()) {
                let mut x: () = (*x);
                unimplemented!();
            }
            n.prefix_inc();
        }
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
    (unsafe {
        for_each_0(
            <types_type_list_float__double__long_double_>::default(),
            <TestFloat>::default(),
        )
    });
    (unsafe {
        for_each_2(
            <types_type_list_short__int__long__long_long_>::default(),
            <TestInt>::default(),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
