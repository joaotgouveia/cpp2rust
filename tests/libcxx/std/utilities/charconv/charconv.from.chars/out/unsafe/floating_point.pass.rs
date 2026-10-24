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
{}pub unsafe fn concat_0 ( mut _a0 : type_list_char__signed_char__short__int__long__long_long____int128_ , mut _a1 : type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ , ) -> type_list_char__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_{
    return <type_list_char__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default()  ;
}
pub static mut all_signed_1: std::cell::LazyCell<
    type_list_char__signed_char__short__int__long__long_long____int128_,
> = std::cell::LazyCell::new(|| unsafe {
    <type_list_char__signed_char__short__int__long__long_long____int128_>::default()
});
pub static mut all_unsigned_2 : std::cell::LazyCell< type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ > = std::cell::LazyCell::new(|| unsafe { <type_list_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ >::default() } ) ;
pub static mut integrals_3 : std::cell::LazyCell< type_list_char__signed_char__short__int__long__long_long____int128__unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long__unsigned___int128_ > = std::cell::LazyCell::new(|| unsafe { ( unsafe { concat_0 ( (*std::cell::LazyCell::force_mut(&mut *&raw mut all_signed_1)) , (*std::cell::LazyCell::force_mut(&mut *&raw mut all_unsigned_2)) , ) } )  } ) ;
pub static mut all_floats_4: std::cell::LazyCell<type_list_float__double_> =
    std::cell::LazyCell::new(|| unsafe { <type_list_float__double_>::default() });
pub unsafe fn run_5(mut _a0: type_list_float__double_) {
    let mut ls: [(); 2] = [
        {
            (unsafe { test_basics_float_::operator_call(&mut <test_basics_float_>::default()) });
            0_()
        },
        {
            (unsafe { test_basics_double_::operator_call(&mut <test_basics_double_>::default()) });
            0_()
        },
    ];
    &(ls);
}
pub unsafe fn run_6(mut _a0: type_list_float__double_) {
    let mut ls: [(); 2] = [
        {
            (unsafe {
                test_scientific_float_::operator_call(&mut <test_scientific_float_>::default())
            });
            0_()
        },
        {
            (unsafe {
                test_scientific_double_::operator_call(&mut <test_scientific_double_>::default())
            });
            0_()
        },
    ];
    &(ls);
}
pub unsafe fn run_7(mut _a0: type_list_float__double_) {
    let mut ls: [(); 2] = [
        {
            (unsafe { test_fixed_float_::operator_call(&mut <test_fixed_float_>::default()) });
            0_()
        },
        {
            (unsafe { test_fixed_double_::operator_call(&mut <test_fixed_double_>::default()) });
            0_()
        },
    ];
    &(ls);
}
pub unsafe fn run_8(mut _a0: type_list_float__double_) {
    let mut ls: [(); 2] = [
        {
            (unsafe { test_general_float_::operator_call(&mut <test_general_float_>::default()) });
            0_()
        },
        {
            (unsafe {
                test_general_double_::operator_call(&mut <test_general_double_>::default())
            });
            0_()
        },
    ];
    &(ls);
}
pub unsafe fn run_9(mut _a0: type_list_float__double_) {
    let mut ls: [(); 2] = [
        {
            (unsafe { test_hex_float_::operator_call(&mut <test_hex_float_>::default()) });
            0_()
        },
        {
            (unsafe { test_hex_double_::operator_call(&mut <test_hex_double_>::default()) });
            0_()
        },
    ];
    &(ls);
}
pub unsafe fn test_infinity_10(mut fmt: ()) {
    let mut s: *const () = c"-InFiNiTyXXX".as_ptr();
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_infinity_11(mut fmt: ()) {
    let mut s: *const () = c"-InFiNiTyXXX".as_ptr();
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_nan_12(mut fmt: ()) {
    {
        let mut s: *const () = c"-NaN(1_A)XXX".as_ptr();
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut s: *const () = c"NaN()".as_ptr();
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        (*unimplemented!()) = ('a' as ());
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        };
        'loop_: for c in IterRange::new(
            c"!@#$%^&*(-=+[]{}|\\;:\'\",./<>?~` \t\x0b\r\n".as_ptr(),
            c"!@#$%^&*(-=+[]{}|\\;:\'\",./<>?~` \t\x0b\r\n"
                .as_ptr()
                .offset((36_()) as isize),
        ) {
            let mut c: () = (*c);
            let mut value: () = 0.25_f64;
            (*unimplemented!()) = c;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_nan_13(mut fmt: ()) {
    {
        let mut s: *const () = c"-NaN(1_A)XXX".as_ptr();
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut s: *const () = c"NaN()".as_ptr();
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: () = unimplemented!();
        (*unimplemented!()) = ('a' as ());
        {
            let mut value: () = 0.25_f64;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        };
        'loop_: for c in IterRange::new(
            c"!@#$%^&*(-=+[]{}|\\;:\'\",./<>?~` \t\x0b\r\n".as_ptr(),
            c"!@#$%^&*(-=+[]{}|\\;:\'\",./<>?~` \t\x0b\r\n"
                .as_ptr()
                .offset((36_()) as isize),
        ) {
            let mut c: () = (*c);
            let mut value: () = 0.25_f64;
            (*unimplemented!()) = c;
            let mut result: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_fmt_independent_14(mut fmt: ()) {
    (unsafe { test_infinity_10(fmt) });
    (unsafe { test_nan_12(fmt) });
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: *const () = c"-".as_ptr();
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: *const () = c".".as_ptr();
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: *const () = c"-.".as_ptr();
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: *const () = c"+0.25".as_ptr();
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_fmt_independent_15(mut fmt: ()) {
    (unsafe { test_infinity_11(fmt) });
    (unsafe { test_nan_13(fmt) });
    {
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: *const () = c"-".as_ptr();
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: *const () = c".".as_ptr();
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: *const () = c"-.".as_ptr();
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut value: () = 0.25_f64;
        let mut s: *const () = c"+0.25".as_ptr();
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_float_ {}
impl test_basics_float_ {
    pub unsafe fn operator_call(&mut self) {
        'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut fmt: () = (*fmt);
            (unsafe { test_fmt_independent_14(fmt) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_basics_double_ {}
impl test_basics_double_ {
    pub unsafe fn operator_call(&mut self) {
        'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
            let mut fmt: () = (*fmt);
            (unsafe { test_fmt_independent_15(fmt) });
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_fixed_float_ {}
impl test_fixed_float_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 0.25_f64;
        {
            let mut s: () = unimplemented!();
            'loop_: for c in IterRange::new( c"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr()  , c"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr() . offset ( ( 87_() ) as isize )  ) { let mut c : () = ( * c  ) ;
  ;
 ;
 ( * unimplemented!()  )   = c  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: *const () = c"001x".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c".5".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-.5".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25.78".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5E10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e+10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e-10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"1.5e+".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e-".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.5e +1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e+ 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e -1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e- 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"1.25e0e12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"1.25e++12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e+-12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e-+12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e--12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"1.25e0x12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"20040229".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456e3".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"0.111111111111111111111111111111111111111111".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () =
                    c"111111111111.111111111111111111111111111111111111111111".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"-0.25".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_fixed_double_ {}
impl test_fixed_double_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 0.25_f64;
        {
            let mut s: () = unimplemented!();
            'loop_: for c in IterRange::new( c"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr()  , c"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr() . offset ( ( 87_() ) as isize )  ) { let mut c : () = ( * c  ) ;
  ;
 ;
 ( * unimplemented!()  )   = c  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: *const () = c"001x".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c".5".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-.5".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25.78".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5E10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e+10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e-10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"1.5e+".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e-".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.5e +1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e+ 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e -1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e- 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"1.25e0e12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"1.25e++12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e+-12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e-+12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e--12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"1.25e0x12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"20040229".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456e3".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"0.111111111111111111111111111111111111111111".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () =
                    c"111111111111.111111111111111111111111111111111111111111".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"-0.25".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_scientific_float_ {}
impl test_scientific_float_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 0.25_f64;
        {
            let mut s: () = unimplemented!();
            'loop_: for c in IterRange::new( c"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr()  , c"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr() . offset ( ( 87_() ) as isize )  ) { let mut c : () = ( * c  ) ;
  ;
 ;
 ( * unimplemented!()  )   = c  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: *const () = c"1.23".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.23e".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"1.5e+".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e-".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.5e +1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e+ 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e -1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e- 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.25e++12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e+-12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e-+12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e--12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"001e0x".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25e0.78".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5E10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e+10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e-10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25e0x12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25e0e12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"20040229e0".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456e3".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"0.111111111111111111111111111111111111111111e0".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () =
                    c"111111111111.111111111111111111111111111111111111111111e0".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"-0.25e0".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1e9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-1e9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1e-9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-1e-9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_scientific_double_ {}
impl test_scientific_double_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 0.25_f64;
        {
            let mut s: () = unimplemented!();
            'loop_: for c in IterRange::new( c"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr()  , c"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr() . offset ( ( 87_() ) as isize )  ) { let mut c : () = ( * c  ) ;
  ;
 ;
 ( * unimplemented!()  )   = c  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: *const () = c"1.23".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.23e".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"1.5e+".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e-".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.5e +1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e+ 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e -1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e- 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.25e++12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e+-12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e-+12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e--12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"001e0x".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25e0.78".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5E10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e+10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e-10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25e0x12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25e0e12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"20040229e0".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456e3".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"0.111111111111111111111111111111111111111111e0".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () =
                    c"111111111111.111111111111111111111111111111111111111111e0".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"-0.25e0".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1e9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-1e9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1e-9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-1e-9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_general_float_ {}
impl test_general_float_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 0.25_f64;
        {
            let mut s: () = unimplemented!();
            'loop_: for c in IterRange::new( c"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr()  , c"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr() . offset ( ( 87_() ) as isize )  ) { let mut c : () = ( * c  ) ;
  ;
 ;
 ( * unimplemented!()  )   = c  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: *const () = c"001x".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c".5e0".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-.5e0".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c".5".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-.5".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25.78".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5E10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e+10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e-10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"1.5e+".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e-".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.5e +1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e+ 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e -1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e- 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.25e++12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e+-12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e-+12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e--12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"1.25e0x12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25e0e12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"20040229".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456e3".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"0.111111111111111111111111111111111111111111".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () =
                    c"111111111111.111111111111111111111111111111111111111111".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"-0.25".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1e9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-1e9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1e-9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-1e-9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_general_double_ {}
impl test_general_double_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 0.25_f64;
        {
            let mut s: () = unimplemented!();
            'loop_: for c in IterRange::new( c"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr()  , c"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr() . offset ( ( 87_() ) as isize )  ) { let mut c : () = ( * c  ) ;
  ;
 ;
 ( * unimplemented!()  )   = c  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: *const () = c"001x".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c".5e0".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-.5e0".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c".5".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-.5".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25.78".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5E10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e+10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e-10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5e".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"1.5e+".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e-".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.5e +1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e+ 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e -1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5e- 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.25e++12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e+-12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e-+12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25e--12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"1.25e0x12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25e0e12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"20040229".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456e3".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"0.111111111111111111111111111111111111111111".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () =
                    c"111111111111.111111111111111111111111111111111111111111".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"-0.25".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1e9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-1e9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1e-9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-1e-9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_hex_float_ {}
impl test_hex_float_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 0.25_f64;
        {
            let mut s: () = unimplemented!();
            'loop_: for c in IterRange::new( c"ghijklmnopqrstuvwxyzGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr()  , c"ghijklmnopqrstuvwxyzGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr() . offset ( ( 75_() ) as isize )  ) { let mut c : () = ( * c  ) ;
  ;
 ;
 ( * unimplemented!()  )   = c  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: *const () = c"001x".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c".5p0".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-.5p0".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c".5".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-.5".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25.78".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5p10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5P10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5p+10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5p-10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5p".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"1.5p+".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5p-".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.5p +1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5p+ 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5p -1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5p- 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.25p++12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25p+-12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25p-+12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25p--12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"1.25p0x12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25p0p12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"131CA25".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456p3".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"0.111111111111111111111111111111111111111111".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () =
                    c"111111111111.111111111111111111111111111111111111111111".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"-0.25".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1p9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-1p9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1p-9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-1p-9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_hex_double_ {}
impl test_hex_double_ {
    pub unsafe fn operator_call(&mut self) {
        let mut r: () = unimplemented!();
        let mut x: () = 0.25_f64;
        {
            let mut s: () = unimplemented!();
            'loop_: for c in IterRange::new( c"ghijklmnopqrstuvwxyzGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr()  , c"ghijklmnopqrstuvwxyzGHIJKLMNOPQRSTUVWXYZ`~!@#$%^&*()_=[]{}\\|;:\'\",/<>? \t\x0b\r\n" .as_ptr() . offset ( ( 75_() ) as isize )  ) { let mut c : () = ( * c  ) ;
  ;
 ;
 ( * unimplemented!()  )   = c  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 unimplemented!()  ;
 }
        }
        {
            let mut s: *const () = c"001x".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c".5p0".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-.5p0".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c".5".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-.5".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25.78".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5p10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5P10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5p+10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5p-10".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.5p".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"1.5p+".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5p-".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.5p +1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5p+ 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5p -1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.5p- 1".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            {
                let mut s: *const () = c"1.25p++12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25p+-12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25p-+12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () = c"1.25p--12".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"1.25p0x12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1.25p0p12".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"131CA25".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"123.456p3".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            {
                let mut s: *const () = c"0.111111111111111111111111111111111111111111".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
            {
                let mut s: *const () =
                    c"111111111111.111111111111111111111111111111111111111111".as_ptr();
                unimplemented!();
                unimplemented!();
                unimplemented!();
                unimplemented!();
            }
        }
        {
            let mut s: *const () = c"-0.25".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1p9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-1p9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"1p-9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut s: *const () = c"-1p-9999999999999999999999999999999999999999".as_ptr();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
}
pub unsafe fn test_random_errors_16() {
    {
        let mut s: *const () = c"4.219902180869891e-2788".as_ptr();
        let mut last: *const () = s
            .offset((unsafe { strlen_17(s) }) as isize)
            .offset(-((1_()) as isize));
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut s: *const () = c"7.411412e-39U".as_ptr();
        let mut last: *const () = s
            .offset((unsafe { strlen_17(s) }) as isize)
            .offset(-((1_()) as isize));
        let mut value: () = 0.25_f64;
        let mut result: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { run_5((*std::cell::LazyCell::force_mut(&mut *&raw mut all_floats_4))) });
    (unsafe { run_6((*std::cell::LazyCell::force_mut(&mut *&raw mut all_floats_4))) });
    (unsafe { run_7((*std::cell::LazyCell::force_mut(&mut *&raw mut all_floats_4))) });
    (unsafe { run_8((*std::cell::LazyCell::force_mut(&mut *&raw mut all_floats_4))) });
    (unsafe { run_9((*std::cell::LazyCell::force_mut(&mut *&raw mut all_floats_4))) });
    (unsafe { test_random_errors_16() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const all_signed_1);
    std::cell::LazyCell::force(&*&raw const all_unsigned_2);
    std::cell::LazyCell::force(&*&raw const integrals_3);
    std::cell::LazyCell::force(&*&raw const all_floats_4);
}
