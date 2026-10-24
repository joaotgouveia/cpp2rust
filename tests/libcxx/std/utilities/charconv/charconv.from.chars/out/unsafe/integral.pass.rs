extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn is_non_narrowing_0(mut a: ()) -> () {
    return unimplemented!();
}
pub unsafe fn is_non_narrowing_1(mut a: ()) -> () {
    return unimplemented!();
}
pub unsafe fn is_non_narrowing_2(mut a: ()) -> () {
    return unimplemented!();
}
pub unsafe fn is_non_narrowing_3(mut a: ()) -> () {
    return unimplemented!();
}
pub unsafe fn is_non_narrowing_4(__args: Box<[VaArg]>) -> () {
    return unimplemented!();
}
pub unsafe fn is_non_narrowing_5(__args: Box<[VaArg]>) -> () {
    return unimplemented!();
}
pub unsafe fn is_non_narrowing_6(__args: Box<[VaArg]>) -> () {
    return unimplemented!();
}
pub unsafe fn is_non_narrowing_7(__args: Box<[VaArg]>) -> () {
    return unimplemented!();
}
pub unsafe fn is_non_narrowing_8(__args: Box<[VaArg]>) -> () {
    return unimplemented!();
}
pub unsafe fn is_non_narrowing_9(__args: Box<[VaArg]>) -> () {
    return unimplemented!();
}
pub unsafe fn is_non_narrowing_10(__args: Box<[VaArg]>) -> () {
    return unimplemented!();
}
pub unsafe fn is_non_narrowing_11(__args: Box<[VaArg]>) -> () {
    return unimplemented!();
}
pub unsafe fn is_non_narrowing_12(__args: Box<[VaArg]>) -> () {
    return unimplemented!();
}
pub unsafe fn _fits_in_13(mut _a0: (), mut _a1: (), __args: Box<[VaArg]>) -> () {
    return true;
}
pub unsafe fn _fits_in_14(mut _a0: (), mut _a1: (), __args: Box<[VaArg]>) -> () {
    return true;
}
pub unsafe fn _fits_in_15(mut _a0: (), mut _a1: (), __args: Box<[VaArg]>) -> () {
    return true;
}
pub unsafe fn _fits_in_16(mut _a0: (), mut _a1: (), __args: Box<[VaArg]>) -> () {
    return true;
}
pub unsafe fn _fits_in_17(mut v: (), mut _a1: (), mut _a2: (), mut _a3: ()) -> () {
    return ((unimplemented!()) <= (v)) && ((v) <= (unimplemented!()));
}
pub unsafe fn _fits_in_18(mut v: (), mut _a1: (), mut _a2: (), mut _a3: ()) -> () {
    return ((unimplemented!()) <= (v)) && ((v) <= (unimplemented!()));
}
pub unsafe fn _fits_in_19(mut v: (), mut _a1: (), mut _a2: (), mut _a3: ()) -> () {
    return ((unimplemented!()) <= (v)) && ((v) <= (unimplemented!()));
}
pub unsafe fn _fits_in_20(mut v: (), mut _a1: (), mut _a2: (), mut _a3: ()) -> () {
    return ((0_()) <= (v)) && ((v) <= (unimplemented!()));
}
pub unsafe fn _fits_in_21(mut v: (), mut _a1: (), mut _a2: (), mut _a3: ()) -> () {
    return ((0_()) <= (v)) && ((v) <= (unimplemented!()));
}
pub unsafe fn _fits_in_22(mut v: (), mut _a1: (), mut _a2: (), mut _a3: ()) -> () {
    return ((0_()) <= (v)) && ((v) <= (unimplemented!()));
}
pub unsafe fn _fits_in_23(mut v: (), mut _a1: (), mut _a2: (), mut _a3: ()) -> () {
    return ((0_()) <= (v)) && ((v) <= (unimplemented!()));
}
pub unsafe fn _fits_in_24(mut v: (), mut _a1: (), mut _a2: (), mut _a3: ()) -> () {
    return ((0_()) <= (v)) && ((v) <= (unimplemented!()));
}
pub unsafe fn _fits_in_25(mut v: (), mut _a1: (), mut _a2: (), mut _a3: ()) -> () {
    return ((0_()) <= (v)) && ((v) <= (unimplemented!()));
}
pub unsafe fn fits_in_26(mut v: ()) -> () {
    return (unsafe {
        let _v: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_4(Box::new([(v).into()])) });
        _fits_in_17(_v, _arg1, unimplemented!(), unimplemented!())
    });
}
pub unsafe fn fits_in_27(mut v: ()) -> () {
    return (unsafe {
        let _v: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_5(Box::new([(v).into()])) });
        _fits_in_18(_v, _arg1, unimplemented!(), unimplemented!())
    });
}
pub unsafe fn fits_in_28(mut v: ()) -> () {
    return (unsafe {
        let _v: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_6(Box::new([(v).into()])) });
        _fits_in_19(_v, _arg1, unimplemented!(), unimplemented!())
    });
}
pub unsafe fn fits_in_29(mut v: ()) -> () {
    return (unsafe {
        let _arg0: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_0(v) });
        _fits_in_13(
            _arg0,
            _arg1,
            Box::new([(unimplemented!()).into(), (unimplemented!()).into()]),
        )
    });
}
pub unsafe fn fits_in_30(mut v: ()) -> () {
    return (unsafe {
        let _arg0: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_1(v) });
        _fits_in_14(
            _arg0,
            _arg1,
            Box::new([(unimplemented!()).into(), (unimplemented!()).into()]),
        )
    });
}
pub unsafe fn fits_in_31(mut v: ()) -> () {
    return (unsafe {
        let _arg0: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_2(v) });
        _fits_in_15(
            _arg0,
            _arg1,
            Box::new([(unimplemented!()).into(), (unimplemented!()).into()]),
        )
    });
}
pub unsafe fn fits_in_32(mut v: ()) -> () {
    return (unsafe {
        let _arg0: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_3(v) });
        _fits_in_16(
            _arg0,
            _arg1,
            Box::new([(unimplemented!()).into(), (unimplemented!()).into()]),
        )
    });
}
pub unsafe fn fits_in_33(mut v: ()) -> () {
    return (unsafe {
        let _v: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_7(Box::new([(v).into()])) });
        _fits_in_20(_v, _arg1, unimplemented!(), unimplemented!())
    });
}
pub unsafe fn fits_in_34(mut v: ()) -> () {
    return (unsafe {
        let _v: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_8(Box::new([(v).into()])) });
        _fits_in_21(_v, _arg1, unimplemented!(), unimplemented!())
    });
}
pub unsafe fn fits_in_35(mut v: ()) -> () {
    return (unsafe {
        let _v: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_9(Box::new([(v).into()])) });
        _fits_in_22(_v, _arg1, unimplemented!(), unimplemented!())
    });
}
pub unsafe fn fits_in_36(mut v: ()) -> () {
    return (unsafe {
        let _v: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_10(Box::new([(v).into()])) });
        _fits_in_23(_v, _arg1, unimplemented!(), unimplemented!())
    });
}
pub unsafe fn fits_in_37(mut v: ()) -> () {
    return (unsafe {
        let _v: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_11(Box::new([(v).into()])) });
        _fits_in_24(_v, _arg1, unimplemented!(), unimplemented!())
    });
}
pub unsafe fn fits_in_38(mut v: ()) -> () {
    return (unsafe {
        let _v: () = v;
        let _arg1: () = (unsafe { is_non_narrowing_12(Box::new([(v).into()])) });
        _fits_in_25(_v, _arg1, unimplemented!(), unimplemented!())
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct type_list_char__signed_char__short__int__long__long_long____int128_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct type_list_char__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct type_list_float__double_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct type_concat_type_list_char__signed_char__short__int__long__long_long____int128___type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128__
{}pub unsafe fn concat_39 ( mut _a0 : type_list_char__signed_char__short__int__long__long_long____int128_ , mut _a1 : type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ , ) -> type_list_char__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_{
    return <type_list_char__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default()  ;
}
pub static mut all_signed_40: std::cell::LazyCell<
    type_list_char__signed_char__short__int__long__long_long____int128_,
> = std::cell::LazyCell::new(|| unsafe {
    <type_list_char__signed_char__short__int__long__long_long____int128_>::default()
});
pub static mut all_unsigned_41 : std::cell::LazyCell< type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ > = std::cell::LazyCell::new(|| unsafe { <type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() } ) ;
pub static mut integrals_42 : std::cell::LazyCell< type_list_char__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ > = std::cell::LazyCell::new(|| unsafe { ( unsafe { concat_39 ( (*std::cell::LazyCell::force_mut(&mut *&raw mut all_signed_40)) , (*std::cell::LazyCell::force_mut(&mut *&raw mut all_unsigned_41)) , ) } )  } ) ;
pub static mut all_floats_43: std::cell::LazyCell<type_list_float__double_> =
    std::cell::LazyCell::new(|| unsafe { <type_list_float__double_>::default() });
pub unsafe fn run_44(
    mut _a0 : type_list_char__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_,
) {
    let mut ls: [(); 13] = [
        {
            (unsafe { test_basics_char_::operator_call(&mut <test_basics_char_>::default()) });
            0_()
        },
        {
            (unsafe {
                test_basics_signed_char_::operator_call(&mut <test_basics_signed_char_>::default())
            });
            0_()
        },
        {
            (unsafe { test_basics_short_::operator_call(&mut <test_basics_short_>::default()) });
            0_()
        },
        {
            (unsafe { test_basics_int_::operator_call(&mut <test_basics_int_>::default()) });
            0_()
        },
        {
            (unsafe { test_basics_long_::operator_call(&mut <test_basics_long_>::default()) });
            0_()
        },
        {
            (unsafe {
                test_basics_long_long_::operator_call(&mut <test_basics_long_long_>::default())
            });
            0_()
        },
        {
            (unsafe {
                test_basics___int128_::operator_call(&mut <test_basics___int128_>::default())
            });
            0_()
        },
        {
            (unsafe {
                test_basics_unsigned_char_::operator_call(
                    &mut <test_basics_unsigned_char_>::default(),
                )
            });
            0_()
        },
        {
            (unsafe {
                test_basics_unsigned_short_::operator_call(
                    &mut <test_basics_unsigned_short_>::default(),
                )
            });
            0_()
        },
        {
            (unsafe {
                test_basics_unsigned_int_::operator_call(&mut <test_basics_unsigned_int_>::default())
            });
            0_()
        },
        {
            (unsafe {
                test_basics_unsigned_long_::operator_call(
                    &mut <test_basics_unsigned_long_>::default(),
                )
            });
            0_()
        },
        {
            (unsafe {
                test_basics_unsigned_long_long_::operator_call(
                    &mut <test_basics_unsigned_long_long_>::default(),
                )
            });
            0_()
        },
        {
            (unsafe {
                test_basics_unsigned___int128_::operator_call(
                    &mut <test_basics_unsigned___int128_>::default(),
                )
            });
            0_()
        },
    ];
    &(ls);
}
pub unsafe fn run_45(mut _a0: type_list_char__signed_char__short__int__long__long_long____int128_) {
    let mut ls: [(); 7] = [
        {
            (unsafe { test_signed_char_::operator_call(&mut <test_signed_char_>::default()) });
            0_()
        },
        {
            (unsafe {
                test_signed_signed_char_::operator_call(&mut <test_signed_signed_char_>::default())
            });
            0_()
        },
        {
            (unsafe { test_signed_short_::operator_call(&mut <test_signed_short_>::default()) });
            0_()
        },
        {
            (unsafe { test_signed_int_::operator_call(&mut <test_signed_int_>::default()) });
            0_()
        },
        {
            (unsafe { test_signed_long_::operator_call(&mut <test_signed_long_>::default()) });
            0_()
        },
        {
            (unsafe {
                test_signed_long_long_::operator_call(&mut <test_signed_long_long_>::default())
            });
            0_()
        },
        {
            (unsafe {
                test_signed___int128_::operator_call(&mut <test_signed___int128_>::default())
            });
            0_()
        },
    ];
    &(ls);
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_char_ {}
impl test_basics_char_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_signed_char_ {}
impl test_basics_signed_char_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_short_ {}
impl test_basics_short_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_int_ {}
impl test_basics_int_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_long_ {}
impl test_basics_long_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_long_long_ {}
impl test_basics_long_long_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics___int128_ {}
impl test_basics___int128_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_unsigned_char_ {}
impl test_basics_unsigned_char_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_unsigned_short_ {}
impl test_basics_unsigned_short_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_unsigned_int_ {}
impl test_basics_unsigned_int_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_unsigned_long_ {}
impl test_basics_unsigned_long_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_unsigned_long_long_ {}
impl test_basics_unsigned_long_long_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_unsigned___int128_ {}
impl test_basics_unsigned___int128_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = unimplemented!();
        {
            let mut s: [(); 5] = std::mem::transmute(*b"001x\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 31] = std::mem::transmute(*b"0X7BAtSGHDkEIXZgQRfYChLpOzRnM \0");
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
                unimplemented!();
            }
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_signed_char_ {}
impl test_signed_char_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 42_();
        {
            let mut s: [(); 7] = std::mem::transmute(*b"- 9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 5] = std::mem::transmute(*b"9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 3] = std::mem::transmute(*b"12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 4] = std::mem::transmute(*b"+30\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_signed_signed_char_ {}
impl test_signed_signed_char_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 42_();
        {
            let mut s: [(); 7] = std::mem::transmute(*b"- 9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 5] = std::mem::transmute(*b"9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 3] = std::mem::transmute(*b"12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 4] = std::mem::transmute(*b"+30\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_signed_short_ {}
impl test_signed_short_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 42_();
        {
            let mut s: [(); 7] = std::mem::transmute(*b"- 9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 5] = std::mem::transmute(*b"9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 3] = std::mem::transmute(*b"12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 4] = std::mem::transmute(*b"+30\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_signed_int_ {}
impl test_signed_int_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 42_();
        {
            let mut s: [(); 7] = std::mem::transmute(*b"- 9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 5] = std::mem::transmute(*b"9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 3] = std::mem::transmute(*b"12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 4] = std::mem::transmute(*b"+30\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_signed_long_ {}
impl test_signed_long_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 42_();
        {
            let mut s: [(); 7] = std::mem::transmute(*b"- 9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 5] = std::mem::transmute(*b"9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 3] = std::mem::transmute(*b"12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 4] = std::mem::transmute(*b"+30\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_signed_long_long_ {}
impl test_signed_long_long_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 42_();
        {
            let mut s: [(); 7] = std::mem::transmute(*b"- 9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 5] = std::mem::transmute(*b"9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 3] = std::mem::transmute(*b"12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 4] = std::mem::transmute(*b"+30\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_signed___int128_ {}
impl test_signed___int128_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 42_();
        {
            let mut s: [(); 7] = std::mem::transmute(*b"- 9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 5] = std::mem::transmute(*b"9+12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 3] = std::mem::transmute(*b"12\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: [(); 4] = std::mem::transmute(*b"+30\0");
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_46() -> () {
    (unsafe { run_44((*std::cell::LazyCell::force_mut(&mut *&raw mut integrals_42))) });
    (unsafe { run_45((*std::cell::LazyCell::force_mut(&mut *&raw mut all_signed_40))) });
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
    (unsafe { test_46() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const all_signed_40);
    std::cell::LazyCell::force(&*&raw const all_unsigned_41);
    std::cell::LazyCell::force(&*&raw const integrals_42);
    std::cell::LazyCell::force(&*&raw const all_floats_43);
}
