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
pub struct MultiStringType {
    pub w_: *const (),
    pub wn_: (),
    pub u8_: *const (),
    pub u8n_: (),
    pub u16_: *const (),
    pub u16n_: (),
    pub u32_: *const (),
    pub u32n_: (),
    pub s_: *const (),
    pub sn_: (),
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const (),
        mut wn: (),
        mut u8: *const (),
        mut u8n: (),
        mut u16: *const (),
        mut u16n: (),
        mut u32: *const (),
        mut u32n: (),
        mut s: *const (),
        mut sn: (),
    ) -> Self {
        let mut this = Self {
            w_: w,
            wn_: wn,
            u8_: u8,
            u8n_: u8n,
            u16_: u16,
            u16n_: u16n,
            u32_: u32,
            u32n_: u32n,
            s_: s,
            sn_: sn,
        };
        this
    }
    pub unsafe fn as_ptr_2(&self, mut _a0: *const ()) -> *const () {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const ()) -> *const () {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const ()) -> *const () {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const ()) -> *const () {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const ()) -> *const () {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const ()) -> () {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const ()) -> () {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const ()) -> () {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const ()) -> () {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const ()) -> () {
        return self.u32n_;
    }
    pub unsafe fn to_pconst_12(&self) -> *const () {
        return self.s_;
    }
    pub unsafe fn to_pconst_13(&self) -> *const () {
        return self.w_;
    }
    pub unsafe fn to_pconst_14(&self) -> *const () {
        return self.u8_;
    }
    pub unsafe fn to_pconst_15(&self) -> *const () {
        return self.u16_;
    }
    pub unsafe fn to_pconst_16(&self) -> *const () {
        return self.u32_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct context_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct context_wchar_t_ {}
pub type status = u16;
pub const status_foo: status = 43690;
pub const status_bar: status = 21845;
pub const status_foobar: status = 43605;
pub unsafe fn operator_cmp_0(
    lhs: *const parse_call_validator,
    rhs: *const parse_call_validator,
) -> () {
    return std::cmp::Ord::cmp(&(lhs), &(rhs));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct parse_call_validator_parse_function_not_called {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct parse_call_validator {}
impl std::cmp::Ord for parse_call_validator {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            operator_cmp_0(
                self as *const parse_call_validator,
                other as *const parse_call_validator,
            )
        }
    }
}
impl std::cmp::PartialOrd for parse_call_validator {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for parse_call_validator {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_cmp_0(
                self as *const parse_call_validator,
                other as *const parse_call_validator,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for parse_call_validator {}
pub unsafe fn get_colons_1() -> () {
    return unimplemented!();
}
pub unsafe fn get_colons_3() -> () {
    return unimplemented!();
}
pub unsafe fn get_colons_5() -> () {
    return unimplemented!();
}
pub unsafe fn get_colons_7() -> () {
    return unimplemented!();
}
pub unsafe fn get_format_types_9() -> () {
    return unimplemented!();
}
pub unsafe fn fmt_invalid_types_10(mut valid: ()) -> () {
    let mut result: () = unimplemented!();
    (unsafe {
        let mut ___r: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_11))(&mut ___r, unimplemented!())
    });
    return unimplemented!();
}
pub unsafe fn fmt_invalid_types_12(mut valid: ()) -> () {
    let mut result: () = unimplemented!();
    (unsafe {
        let mut ___r: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_11))(&mut ___r, unimplemented!())
    });
    return unimplemented!();
}
pub unsafe fn fmt_invalid_types_13(mut valid: ()) -> () {
    let mut result: () = unimplemented!();
    (unsafe {
        let mut ___r: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_11))(&mut ___r, unimplemented!())
    });
    return unimplemented!();
}
pub unsafe fn fmt_invalid_types_14(mut valid: ()) -> () {
    let mut result: () = unimplemented!();
    (unsafe {
        let mut ___r: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut copy_11))(&mut ___r, unimplemented!())
    });
    return unimplemented!();
}
static mut result_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut result_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut result_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
static mut result_8: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub unsafe fn fmt_invalid_types_15(mut valid: ()) -> () {
    return (unsafe { fmt_invalid_types_12(unimplemented!()) });
}
pub unsafe fn fmt_invalid_types_16(mut valid: ()) -> () {
    return (unsafe { fmt_invalid_types_10(unimplemented!()) });
}
pub unsafe fn fmt_invalid_nested_types_17(mut valid: ()) -> () {
    return (unsafe { fmt_invalid_types_13(unimplemented!()) });
}
pub unsafe fn fmt_invalid_nested_types_18(mut valid: ()) -> () {
    return (unsafe { fmt_invalid_types_14(unimplemented!()) });
}
pub unsafe fn get_temp_file_name_19() -> () {
    let mut Name: () = unimplemented!();
    let mut FD: () = (unsafe { mkstemp_20((&mut (*unimplemented!()) as *mut ())) });
    if ((FD) == (-1_())) {
        (unsafe { perror_21(c"mkstemp".as_ptr()) });
        unimplemented!();
    }
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn glibc_version_less_than_22(mut version: *const ()) -> () {
    let mut test_version: () = unimplemented!();
    let mut n: () = (unsafe {
        confstr_23(
            (anon_24__CS_GNU_LIBC_VERSION as ()),
            std::ptr::null_mut(),
            0_(),
        )
    });
    let mut current_version: *mut () =
        Box::leak((0..n).map(|_| unimplemented!()).collect::<Box<[()]>>()).as_mut_ptr();
    (unsafe { confstr_23((anon_24__CS_GNU_LIBC_VERSION as ()), current_version, n) });
    let mut result: () =
        ((unsafe { strverscmp_25((current_version).cast_const(), unimplemented!()) }) < (0_()));
    {
        let __p = current_version;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                __p,
                libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<()>(),
            )))
        }
    };
    return result;
}
pub unsafe fn test_char_26(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_0: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_0;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(0).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_1: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_1;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(0).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 44_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int__int_  = unimplemented!() ;
        let mut _args_3: () = 44_();
        let mut _args_4: () = 11_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(2) .call ( _expected , _fmt , & mut input , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(0).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let mut _arg3: () = 44_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(unimplemented!(), unimplemented!(), &mut input, &mut _arg3)
    });
}
pub unsafe fn test_char_27(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_2: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_2;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(21).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_3: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_3;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(21).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 44_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int__int_  = unimplemented!() ;
        let mut _args_3: () = 44_();
        let mut _args_4: () = 11_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(34) .call ( _expected , _fmt , & mut input , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(21).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let mut _arg3: () = 44_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(22)
            .call(unimplemented!(), unimplemented!(), &mut input, &mut _arg3)
    });
}
pub unsafe fn test_char_to_wchar_28(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_4: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_4;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(2).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_5: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_5;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(2).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 44_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int__int_  = unimplemented!() ;
        let mut _args_3: () = 44_();
        let mut _args_4: () = 11_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(5) .call ( _expected , _fmt , & mut input , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(2).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let mut _arg3: () = 44_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(3)
            .call(unimplemented!(), unimplemented!(), &mut input, &mut _arg3)
    });
}
pub unsafe fn test_bool_29(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_6: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_6;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(4)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_7: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_7;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(4)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        let mut _args_4: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(8) .call ( _expected , _fmt , _args_2 , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        let mut _arg3: () = 41_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(5)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn test_bool_30(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_8: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_8;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(6)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_9: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_9;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(6)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        let mut _args_4: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(11) .call ( _expected , _fmt , _args_2 , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        let mut _arg3: () = 41_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn test_bool_31(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_10: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_10;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(23)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_11: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_11;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(23)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        let mut _args_4: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(37) .call ( _expected , _fmt , _args_2 , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        let mut _arg3: () = 41_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(24)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn test_bool_32(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_12: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_12;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(25)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_13: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_13;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(25)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        let mut _args_4: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(40) .call ( _expected , _fmt , _args_2 , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        let mut _arg3: () = 41_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(26)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn test_bool_33(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let mut _input: () = unimplemented!();
        test_bool_29(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_bool_30(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_bool_34(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let mut _input: () = unimplemented!();
        test_bool_31(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_bool_32(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_int_35(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_14: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_14;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(8)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_15: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_15;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(8)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        let mut _args_4: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(14) .call ( _expected , _fmt , _args_2 , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        let mut _arg3: () = 41_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(9)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn test_int_36(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_16: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_16;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_17: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_17;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        let mut _args_4: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(17) .call ( _expected , _fmt , _args_2 , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        let mut _arg3: () = 41_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(11)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn test_int_37(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut adaptor,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref__int_, *mut adaptor, *mut ())>(
                30,
            )
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref__int_, *mut adaptor, *mut ())>(
                30,
            )
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref__int_, *mut adaptor, *mut ())>(
                30,
            )
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref__int_, *mut adaptor, *mut ())>(
                30,
            )
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_18: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_18;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut adaptor = input;
            check_exception.spec::<fn((), (), *mut adaptor)>(19).call(
                unimplemented!(),
                _arg1,
                _arg2,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref__int_, *mut adaptor, *mut ())>(
                30,
            )
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref__int_, *mut adaptor, *mut ())>(
                30,
            )
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref__int_, *mut adaptor, *mut ())>(
                30,
            )
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref__int_, *mut adaptor, *mut ())>(
                30,
            )
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_19: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_19;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut adaptor = input;
            check_exception.spec::<fn((), (), *mut adaptor)>(19).call(
                unimplemented!(),
                _arg1,
                _arg2,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 41_();
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref__int_, *mut adaptor, *mut ())>(
                30,
            )
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 41_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_char__adaptor_ref__int__int_,
                *mut adaptor,
                *mut (),
                *mut (),
            )>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(19)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        let mut _arg3: () = 41_();
        check_exception
            .spec::<fn((), (), *mut adaptor, *mut ())>(20)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn test_int_38(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_20: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_20;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(27)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_21: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_21;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(27)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        let mut _args_4: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(43) .call ( _expected , _fmt , _args_2 , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        let mut _arg3: () = 41_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(28)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn test_int_39(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_22: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_22;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(29)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_23: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_23;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(29)
                .call(unimplemented!(), _arg1, _arg2)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = unimplemented!() ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int__int_  = unimplemented!() ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        let mut _args_4: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(46) .call ( _expected , _fmt , _args_2 , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut () = input;
        let mut _arg3: () = 41_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(30)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn test_int_40(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut adaptor,
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_24: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_24;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut adaptor = input;
            check_exception.spec::<fn((), (), *mut adaptor)>(38).call(
                unimplemented!(),
                _arg1,
                _arg2,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    let mut __tmp_25: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_25;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _arg1: () = unimplemented!();
            let _arg2: *mut adaptor = input;
            check_exception.spec::<fn((), (), *mut adaptor)>(38).call(
                unimplemented!(),
                _arg1,
                _arg2,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = unimplemented!();
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int__int_ = unimplemented!();
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 41_();
        let mut _args_4: () = 10_();
        check
            .spec::<fn(
                (),
                std_basic_format_string_wchar_t__adaptor_ref__int__int_,
                *mut adaptor,
                *mut (),
                *mut (),
            )>(60)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        check_exception
            .spec::<fn((), (), *mut adaptor)>(38)
            .call(unimplemented!(), _arg1, _arg2)
    });
    (unsafe {
        let _arg1: () = unimplemented!();
        let _arg2: *mut adaptor = input;
        let mut _arg3: () = 41_();
        check_exception
            .spec::<fn((), (), *mut adaptor, *mut ())>(39)
            .call(unimplemented!(), _arg1, _arg2, &mut _arg3)
    });
}
pub unsafe fn test_int_41(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let mut _input: () = unimplemented!();
        test_int_35(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_int_36(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_int_42(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let mut _input: () = unimplemented!();
        test_int_38(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_int_39(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_floating_point_43(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_26: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_26;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(12).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_27: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_27;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(12).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 29_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int__int_  = unimplemented!() ;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(20) .call ( _expected , _fmt , & mut input , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let mut _arg3: () = 29_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(unimplemented!(), unimplemented!(), &mut input, &mut _arg3)
    });
}
pub unsafe fn test_floating_point_44(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_28: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_28;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(31).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_29: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_29;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(31).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 29_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int__int_  = unimplemented!() ;
        let mut _args_3: () = 29_();
        let mut _args_4: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(49) .call ( _expected , _fmt , & mut input , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(31).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let mut _arg3: () = 29_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(32)
            .call(unimplemented!(), unimplemented!(), &mut input, &mut _arg3)
    });
}
pub unsafe fn test_pointer_45(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_30: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_30;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(14).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_31: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_31;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(14).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 18_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int__int_  = unimplemented!() ;
        let mut _args_3: () = 18_();
        let mut _args_4: () = 11_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(23) .call ( _expected , _fmt , & mut input , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let mut _arg3: () = 18_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(15)
            .call(unimplemented!(), unimplemented!(), &mut input, &mut _arg3)
    });
}
pub unsafe fn test_pointer_46(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_32: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_32;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(33).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_33: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_33;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(33).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 18_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int__int_  = unimplemented!() ;
        let mut _args_3: () = 18_();
        let mut _args_4: () = 11_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(52) .call ( _expected , _fmt , & mut input , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(33).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let mut _arg3: () = 18_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(unimplemented!(), unimplemented!(), &mut input, &mut _arg3)
    });
}
pub unsafe fn test_string_47(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_34: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_34;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(16).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_35: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_35;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(16).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 51_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int__int_  = unimplemented!() ;
        let mut _args_3: () = 51_();
        let mut _args_4: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(26) .call ( _expected , _fmt , & mut input , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(16).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let mut _arg3: () = 51_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(17)
            .call(unimplemented!(), unimplemented!(), &mut input, &mut _arg3)
    });
}
pub unsafe fn test_string_48(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_36: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_36;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(35).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    let mut __tmp_37: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_37;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            check_exception.spec::<fn((), (), *mut ())>(35).call(
                unimplemented!(),
                unimplemented!(),
                &mut input,
            )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 51_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int__int_  = unimplemented!() ;
        let mut _args_3: () = 51_();
        let mut _args_4: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int__int_ ,*mut () ,*mut () ,*mut () ,)>(55) .call ( _expected , _fmt , & mut input , & mut _args_3 , & mut _args_4 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(35).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let mut _arg3: () = 51_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(36)
            .call(unimplemented!(), unimplemented!(), &mut input, &mut _arg3)
    });
}
pub unsafe fn test_status_49(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(28) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(28) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(28) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(28) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(18).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
}
pub unsafe fn test_status_50(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(57) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(57) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(57) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = unimplemented!() ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(57) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(37).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(37).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(37).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(37).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(37).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(37).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(37).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(37).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(37).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = unimplemented!() ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(37).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
    (unsafe {
        check_exception.spec::<fn((), (), *mut ())>(37).call(
            unimplemented!(),
            unimplemented!(),
            &mut input,
        )
    });
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, Default)]
pub struct adaptor {
    data_: (),
}
impl adaptor {
    pub unsafe fn begin(&mut self) -> () {
        return unimplemented!();
    }
    pub unsafe fn end(&mut self) -> () {
        return unimplemented!();
    }
    pub unsafe fn new(data: *mut ()) -> Self {
        let mut this = Self {
            data_: unimplemented!(),
        };
        this
    }
}
const _: () = assert!(true, "std::format_kind<adaptor> == std::range_format::map");
pub unsafe fn test_adaptor_51(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let mut _input: adaptor = {
            let mut __tmp_38: () = unimplemented!();
            adaptor::new({ &mut __tmp_38 })
        };
        test_int_37(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_adaptor_52(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let mut _input: adaptor = {
            let mut __tmp_39: () = unimplemented!();
            adaptor::new({ &mut __tmp_39 })
        };
        test_int_40(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn format_tests_53(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe { test_char_26(check.copy_from(), check_exception.copy_from()) });
    {}
    (unsafe { test_bool_33(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_int_41(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_floating_point_43(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_pointer_45(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_string_47(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_status_49(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_adaptor_51(check.copy_from(), check_exception.copy_from()) });
}
pub unsafe fn format_tests_54(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe { test_char_27(check.copy_from(), check_exception.copy_from()) });
    {
        (unsafe { test_char_to_wchar_28(check.copy_from(), check_exception.copy_from()) });
    }
    (unsafe { test_bool_34(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_int_42(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_floating_point_44(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_pointer_46(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_string_48(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_status_50(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_adaptor_52(check.copy_from(), check_exception.copy_from()) });
}
pub unsafe fn test_eprintf_55(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const *const (),
    args_3: *const *const (),
    args_4: *const (),
    args_5: *const *const (),
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
            (*args_3),
            (*args_4),
            (*args_5),
        )
    });
}
pub unsafe fn test_eprintf_56(
    mut fmt: *const (),
    args_1: *const *const (),
    args_2: *const *const (),
    args_3: *const (),
) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args_1),
            (*args_2),
            (*args_3),
        )
    });
}
pub unsafe fn test_eprintf_57(mut fmt: *const (), args: *const *const ()) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args),
        )
    });
}
pub unsafe fn test_log_58(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    mut message: *const (),
) {
    let mut msg: *const () = if !(condition).is_null() {
        c"Assertion failure: ".as_ptr()
    } else {
        c"Unconditional failure:".as_ptr()
    };
    (unsafe {
        test_eprintf_55(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
pub unsafe fn test_log_59(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_56(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_require_60(
    mut condition: (),
    mut condition_str: *const (),
    mut file: *const (),
    mut line: (),
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const () = condition_str;
        let _file: *const () = file;
        let _line: () = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_59(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_is_high_surrogate_61(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (56319_()));
}
pub unsafe fn test_is_low_surrogate_62(mut value: ()) -> () {
    return ((value) >= (56320_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_surrogate_63(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_code_point_64(mut value: ()) -> () {
    return ((value) <= (1114111_()));
}
pub unsafe fn test_is_scalar_value_65(mut value: ()) -> () {
    return (unsafe { test_is_code_point_64(value) })
        && (!(unsafe { test_is_surrogate_63(value) }));
}
pub static mut test_replacement_character_66: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (65533 as ()) });
pub unsafe fn test_encode_67(out_it: *mut (), mut value: ()) {
    if ((value) < (128_())) {
        unimplemented!();
    } else if ((value) < (2048_())) {
        unimplemented!();
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_encode_68(out_it: *mut (), mut value: ()) {
    if (((value) & (4294901760_())) == (0_())) {
        (unsafe {
            let _out_it: *mut () = out_it;
            let _value: () = value;
            test_encode_67(_out_it, _value)
        });
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_transcode_69(mut first: (), mut last: (), mut out_it: ()) -> () {
    'loop_: while !(unimplemented!()) {
        let mut value: () = (*unimplemented!());
        if (unsafe { test_is_code_point_64(value) }) {
            (unsafe { test_encode_68(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_67(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_66)),
                )
            });
        }
    }
    return out_it;
}
pub unsafe fn test_transcode_70(mut first: *const (), mut last: *const (), mut out_it: ()) -> () {
    'loop_: while ((first) != (last)) {
        let mut value: () = (*first.postfix_inc());
        if (unsafe { test_is_code_point_64(value) }) {
            (unsafe { test_encode_68(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_67(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_66)),
                )
            });
        }
    }
    return out_it;
}
pub unsafe fn operator_shl_71(os: *mut (), loc: *const ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_72(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_73(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_74(stream: *mut (), value: *const [(); 18]) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_75(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_76(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_77(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if !(unimplemented!()) {
        {}
        (unsafe { test_transcode_69(unimplemented!(), unimplemented!(), unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_78(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ((b) != (e)) {
        {}
        (unsafe { test_transcode_70(b, e, unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_79(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ((b) != (e)) {
        {}
        (unsafe { test_transcode_70(b, e, unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_message_80(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        let mut sstr: () = unimplemented!();
        {
            (unsafe {
                let _stream: *mut () = &mut sstr;
                let _value: *const [(); 18] = args_0;
                test_concat_74(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_76(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_74(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_75(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_74(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_73(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
                                    test_concat_72(_stream, _value)
                                })
                            }
                        }
                    }
                }
            }
        };
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_81(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        let mut sstr: () = unimplemented!();
        {
            (unsafe {
                let _stream: *mut () = &mut sstr;
                let _value: *const [(); 18] = args_0;
                test_concat_74(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_79(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_74(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_78(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_74(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_77(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
                                    test_concat_72(_stream, _value)
                                })
                            }
                        }
                    }
                }
            }
        };
        return unimplemented!();
    }
    panic!("ub: non-void function does not return a value")
}
pub static mut test_82: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(|| unsafe {
    lambda_unsafe! ( Generic , { } , | mut expected : () , mut fmt : std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_60 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_83 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_84 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_85 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_86 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_87 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_88 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_89 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_90 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_91 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_92 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_93 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_94 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_95 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_96 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_97 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_98 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_99 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_100 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_101 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_102 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_103 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_104 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_105 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_106 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_107 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_108 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_109 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_110 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__adaptor_ref_ , args : *mut adaptor , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__adaptor_ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_111 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__adaptor_ref__int_ , args_2 : *mut adaptor , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__adaptor_ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_112 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__adaptor_ref__int__int_ , args_2 : *mut adaptor , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__adaptor_ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_113 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_114 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_115 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_116 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_117 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_118 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_119 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_120 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_121 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_122 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_123 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_124 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_125 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_126 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_127 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_128 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_129 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_130 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_131 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_132 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_133 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_134 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_135 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_136 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_137 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_138 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_139 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__adaptor_ref_ , args : *mut adaptor , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__adaptor_ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_140 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__adaptor_ref__int_ , args_2 : *mut adaptor , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__adaptor_ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_141 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__adaptor_ref__int__int_ , args_2 : *mut adaptor , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__adaptor_ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_57 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_142 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 39_() , & mut _arg , ) } )  ;
 } )
});
pub static mut test_exception_143: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut adaptor| {},
            |mut _a0: (), mut _a1: (), _a2: *mut adaptor, _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut adaptor| {},
            |mut _a0: (), mut _a1: (), _a2: *mut adaptor, _a3: *mut ()| {}
        )
    });

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
        format_tests_53(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_82)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_143)).copy_from(),
        )
    });
    (unsafe {
        format_tests_54(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_82)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_143)).copy_from(),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_66);
    std::cell::LazyCell::force(&*&raw const test_82);
    std::cell::LazyCell::force(&*&raw const test_exception_143);
}
