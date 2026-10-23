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
static mut result_2: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { 1_() },
        { (':' as ()) },
        None,
    )
});
static mut result_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
        { 1_() },
        { (':' as ()) },
        None,
    )
});
static mut result_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { 2_() },
        { (':' as ()) },
        None,
    )
});
static mut result_8: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
        { 2_() },
        { (':' as ()) },
        None,
    )
});
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
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_0 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_0 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_1 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_1 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_2 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_2 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_3 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_3 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_4 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_4 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_5 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_5 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_6 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_6 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_7 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_7 } , )  } ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_8 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_8 } , )  } ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_9 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_9 } , )  } ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_10 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_10 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_11 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_11 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_12 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_12 } , )  } ;
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
    let mut __tmp_13: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_13;
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
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_14 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_14 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_15 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_15 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_16 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_16 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_17 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_17 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_18 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_18 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_19 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_19 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_20 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_20 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_21 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_21 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_22 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_22 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_23 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_23 } , )  } ;
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
    let mut __tmp_24: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_24;
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
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_25 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_25 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(0) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_26 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_26 } , )  } ;
        let mut _args_3: () = 44_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(1) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int__int_  = { let mut __tmp_27 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int__int_ :: new ( { & mut __tmp_27 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_28 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_28 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_29 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_29 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_30 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_30 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_31 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_31 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_32 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_32 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_33 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_33 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_34 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_34 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = { let mut __tmp_35 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ :: new ( { & mut __tmp_35 } , )  } ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = { let mut __tmp_36 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ :: new ( { & mut __tmp_36 } , )  } ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = { let mut __tmp_37 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ :: new ( { & mut __tmp_37 } , )  } ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = { let mut __tmp_38 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ :: new ( { & mut __tmp_38 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_39 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_39 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_40 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_40 } , )  } ;
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
    let mut __tmp_41: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_41;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_42 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_42 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_43 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_43 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_44 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_44 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_45 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_45 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = { let mut __tmp_46 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ :: new ( { & mut __tmp_46 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = { let mut __tmp_47 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ :: new ( { & mut __tmp_47 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = { let mut __tmp_48 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ :: new ( { & mut __tmp_48 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = { let mut __tmp_49 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ :: new ( { & mut __tmp_49 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_50 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_50 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_51 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_51 } , )  } ;
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
    let mut __tmp_52: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_52;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_  = { let mut __tmp_53 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ :: new ( { & mut __tmp_53 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref_ ,*mut () ,)>(32) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_  = { let mut __tmp_54 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ :: new ( { & mut __tmp_54 } , )  } ;
        let mut _args_3: () = 44_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int_ ,*mut () ,*mut () ,)>(33) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int__int_  = { let mut __tmp_55 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_wchar_t__wchar_t__std_less_wchar_t___std_allocator_std_pair_const_wchar_t__wchar_t____ref__int__int_ :: new ( { & mut __tmp_55 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_56 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_56 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_57 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_57 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_58 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_58 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_59 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_59 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_60 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_60 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_61 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_61 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_62 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_62 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_63 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_63 } , )  } ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_64 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_64 } , )  } ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_65 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_65 } , )  } ;
        let mut _args_3: () = 35_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_66 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_66 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_67 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_67 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_68 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_68 } , )  } ;
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
    let mut __tmp_69: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_69;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_70 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_70 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_71 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_71 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_72 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_72 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_73 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_73 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_74 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_74 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_75 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_75 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_76 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_76 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_77 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_77 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_78 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_78 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_79 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_79 } , )  } ;
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
    let mut __tmp_80: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_80;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_  = { let mut __tmp_81 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ :: new ( { & mut __tmp_81 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref_ ,*mut () ,)>(3) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_  = { let mut __tmp_82 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ :: new ( { & mut __tmp_82 } , )  } ;
        let mut _args_3: () = 44_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int_ ,*mut () ,*mut () ,)>(4) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int__int_  = { let mut __tmp_83 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_char__char__std_less_char___std_allocator_std_pair_const_char__char____ref__int__int_ :: new ( { & mut __tmp_83 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_84 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_84 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_85 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_85 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_86 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_86 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_87 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_87 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_88 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_88 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_89 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_89 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_90 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_90 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_91 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_91 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_92 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_92 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_93 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_93 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_94 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_94 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_95 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_95 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_96 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_96 } , )  } ;
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
    let mut __tmp_97: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_97;
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
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_98 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_98 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_99 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_99 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_100 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_100 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_101 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_101 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_102 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_102 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_103 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_103 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_104 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_104 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_105 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_105 } , )  } ;
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
    let mut __tmp_106: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_106;
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
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_107 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_107 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(6) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_108 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_108 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(7) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int__int_  = { let mut __tmp_109 : ()  = unimplemented!() ; std_basic_format_string_char__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int__int_ :: new ( { & mut __tmp_109 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_110 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_110 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_111 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_111 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_112 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_112 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_113 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_113 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_114 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_114 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_115 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_115 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_116 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_116 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_117 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_117 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_118 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_118 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_119 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_119 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_120 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_120 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_121 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_121 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_122 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_122 } , )  } ;
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
    let mut __tmp_123: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_123;
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
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_124 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_124 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_125 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_125 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_126 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_126 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_127 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_127 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_128 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_128 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_129 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_129 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_130 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_130 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_131 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_131 } , )  } ;
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
    let mut __tmp_132: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_132;
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
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_133 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_133 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(9) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_134 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_134 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(10) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int__int_  = { let mut __tmp_135 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int__int_ :: new ( { & mut __tmp_135 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_136 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_136 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_137 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_137 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_138 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_138 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_139 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_139 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_140 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_140 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_141 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_141 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_142 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_142 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_143 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_143 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_144 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_144 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_145 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_145 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_146 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_146 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_147 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_147 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_148 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_148 } , )  } ;
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
    let mut __tmp_149: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_149;
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
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_150 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_150 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_151 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_151 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_152 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_152 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_153 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_153 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_154 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_154 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_155 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_155 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_156 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_156 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_157 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_157 } , )  } ;
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
    let mut __tmp_158: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_158;
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
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_  = { let mut __tmp_159 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ :: new ( { & mut __tmp_159 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref_ ,*mut () ,)>(35) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_  = { let mut __tmp_160 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ :: new ( { & mut __tmp_160 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int_ ,*mut () ,*mut () ,)>(36) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int__int_  = { let mut __tmp_161 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_multimap_bool__int__std_less_bool___std_allocator_std_pair_const_bool__int____ref__int__int_ :: new ( { & mut __tmp_161 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_162 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_162 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_163 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_163 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_164 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_164 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_165 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_165 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_166 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_166 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_167 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_167 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_168 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_168 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_169 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_169 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_170 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_170 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_171 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_171 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 34_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_172 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_172 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_173 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_173 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_174 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_174 } , )  } ;
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
    let mut __tmp_175: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_175;
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
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_176 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_176 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_177 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_177 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_178 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_178 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_179 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_179 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_180 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_180 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_181 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_181 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_182 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_182 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_183 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_183 } , )  } ;
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
    let mut __tmp_184: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_184;
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
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_185 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_185 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(38) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_186 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_186 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(39) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int__int_  = { let mut __tmp_187 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_multimap_bool__int__std_less_bool___std_deque_bool__std_allocator_bool____std_vector_int__std_allocator_int____ref__int__int_ :: new ( { & mut __tmp_187 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_188 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_188 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_189 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_189 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_190 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_190 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_191 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_191 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_192 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_192 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_193 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_193 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_194 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_194 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_195 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_195 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_196 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_196 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_197 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_197 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_198 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_198 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_199 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_199 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_200 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_200 } , )  } ;
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
    let mut __tmp_201: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_201;
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
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_202 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_202 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_203 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_203 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_204 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_204 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_205 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_205 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_206 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_206 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_207 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_207 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_208 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_208 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_209 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_209 } , )  } ;
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
    let mut __tmp_210: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_210;
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
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_211 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_211 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(12) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_212 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_212 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(13) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int__int_  = { let mut __tmp_213 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int__int_ :: new ( { & mut __tmp_213 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_214 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_214 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_215 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_215 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_216 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_216 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_217 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_217 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_218 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_218 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_219 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_219 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_220 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_220 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_221 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_221 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_222 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_222 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_223 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_223 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_224 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_224 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_225 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_225 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_226 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_226 } , )  } ;
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
    let mut __tmp_227: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_227;
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
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_228 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_228 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_229 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_229 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_230 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_230 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_231 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_231 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_232 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_232 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_233 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_233 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_234 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_234 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_235 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_235 } , )  } ;
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
    let mut __tmp_236: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_236;
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
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_237 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_237 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(15) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_238 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_238 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(16) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int__int_  = { let mut __tmp_239 : ()  = unimplemented!() ; std_basic_format_string_char__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int__int_ :: new ( { & mut __tmp_239 } , )  } ;
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
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_240: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_240 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_241: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_241 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_242: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_242 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_243: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_243 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_244: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_244 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_245: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_245 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_246: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_246 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = {
            let mut __tmp_247: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref__int_::new({ &mut __tmp_247 })
        };
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
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = {
            let mut __tmp_248: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref__int_::new({ &mut __tmp_248 })
        };
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
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = {
            let mut __tmp_249: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref__int_::new({ &mut __tmp_249 })
        };
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
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = {
            let mut __tmp_250: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref__int_::new({ &mut __tmp_250 })
        };
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
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_251: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_251 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_252: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_252 })
        };
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
    let mut __tmp_253: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_253;
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
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_254: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_254 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_255: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_255 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_256: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_256 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_257: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_257 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = {
            let mut __tmp_258: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref__int_::new({ &mut __tmp_258 })
        };
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
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = {
            let mut __tmp_259: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref__int_::new({ &mut __tmp_259 })
        };
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
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = {
            let mut __tmp_260: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref__int_::new({ &mut __tmp_260 })
        };
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
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = {
            let mut __tmp_261: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref__int_::new({ &mut __tmp_261 })
        };
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
    let mut __tmp_262: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_262;
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
        let _fmt: std_basic_format_string_char__adaptor_ref_ = {
            let mut __tmp_263: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref_::new({ &mut __tmp_263 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_char__adaptor_ref_, *mut adaptor)>(29)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__adaptor_ref__int_ = {
            let mut __tmp_264: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref__int_::new({ &mut __tmp_264 })
        };
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
        let _fmt: std_basic_format_string_char__adaptor_ref__int__int_ = {
            let mut __tmp_265: () = unimplemented!();
            std_basic_format_string_char__adaptor_ref__int__int_::new({ &mut __tmp_265 })
        };
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
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_266 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_266 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_267 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_267 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_268 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_268 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_269 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_269 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_270 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_270 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_271 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_271 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_272 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_272 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_273 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_273 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_274 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_274 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_275 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_275 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_276 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_276 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_277 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_277 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_278 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_278 } , )  } ;
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
    let mut __tmp_279: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_279;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_280 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_280 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_281 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_281 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_282 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_282 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_283 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_283 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_284 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_284 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_285 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_285 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_286 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_286 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_287 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_287 } , )  } ;
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
    let mut __tmp_288: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_288;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_  = { let mut __tmp_289 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ :: new ( { & mut __tmp_289 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref_ ,*mut () ,)>(41) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_  = { let mut __tmp_290 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ :: new ( { & mut __tmp_290 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int_ ,*mut () ,*mut () ,)>(42) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int__int_  = { let mut __tmp_291 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_int__int__std_less_int___std_allocator_std_pair_const_int__int____ref__int__int_ :: new ( { & mut __tmp_291 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_292 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_292 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_293 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_293 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_294 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_294 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_295 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_295 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_296 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_296 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_297 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_297 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_298 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_298 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_299 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_299 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_300 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_300 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_301 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_301 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_302 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_302 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_303 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_303 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_304 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_304 } , )  } ;
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
    let mut __tmp_305: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_305;
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
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_306 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_306 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_307 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_307 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_308 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_308 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_309 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_309 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_310 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_310 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_311 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_311 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_312 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_312 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_313 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_313 } , )  } ;
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
    let mut __tmp_314: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_314;
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
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_  = { let mut __tmp_315 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ :: new ( { & mut __tmp_315 } , )  } ;
        let _args: *mut () = input;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref_ ,*mut () ,)>(44) .call ( _expected , _fmt , _args , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_  = { let mut __tmp_316 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ :: new ( { & mut __tmp_316 } , )  } ;
        let _args_2: *mut () = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int_ ,*mut () ,*mut () ,)>(45) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int__int_  = { let mut __tmp_317 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_flat_map_int__int__std_less_int___std_vector_int__std_allocator_int____std_vector_int__std_allocator_int____ref__int__int_ :: new ( { & mut __tmp_317 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_318: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_318 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_319: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_319 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_320: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_320 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_321: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_321 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_322: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_322 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_323: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_323 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_324: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_324 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = {
            let mut __tmp_325: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref__int_::new({ &mut __tmp_325 })
        };
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = {
            let mut __tmp_326: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref__int_::new({ &mut __tmp_326 })
        };
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = {
            let mut __tmp_327: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref__int_::new({ &mut __tmp_327 })
        };
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 30_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = {
            let mut __tmp_328: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref__int_::new({ &mut __tmp_328 })
        };
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
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_329: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_329 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_330: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_330 })
        };
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
    let mut __tmp_331: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_331;
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
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_332: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_332 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_333: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_333 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_334: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_334 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_335: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_335 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = {
            let mut __tmp_336: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref__int_::new({ &mut __tmp_336 })
        };
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = {
            let mut __tmp_337: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref__int_::new({ &mut __tmp_337 })
        };
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = {
            let mut __tmp_338: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref__int_::new({ &mut __tmp_338 })
        };
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = {
            let mut __tmp_339: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref__int_::new({ &mut __tmp_339 })
        };
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
    let mut __tmp_340: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_340;
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
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref_ = {
            let mut __tmp_341: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref_::new({ &mut __tmp_341 })
        };
        let _args: *mut adaptor = input;
        check
            .spec::<fn((), std_basic_format_string_wchar_t__adaptor_ref_, *mut adaptor)>(58)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int_ = {
            let mut __tmp_342: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref__int_::new({ &mut __tmp_342 })
        };
        let _args_2: *mut adaptor = input;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__adaptor_ref__int_ ,*mut adaptor ,*mut () ,)>(59) .call ( _expected , _fmt , _args_2 , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__adaptor_ref__int__int_ = {
            let mut __tmp_343: () = unimplemented!();
            std_basic_format_string_wchar_t__adaptor_ref__int__int_::new({ &mut __tmp_343 })
        };
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
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_344 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_344 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_345 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_345 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_346 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_346 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_347 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_347 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_348 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_348 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_349 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_349 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_350 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_350 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_351 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_351 } , )  } ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_352 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_352 } , )  } ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_353 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_353 } , )  } ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_354 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_354 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_355 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_355 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_356 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_356 } , )  } ;
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
    let mut __tmp_357: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_357;
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
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_358 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_358 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_359 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_359 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_360 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_360 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_361 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_361 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_362 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_362 } , )  } ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_363 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_363 } , )  } ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_364 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_364 } , )  } ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_365 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_365 } , )  } ;
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
    let mut __tmp_366: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_366;
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
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_367 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_367 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(18) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_368 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_368 } , )  } ;
        let mut _args_3: () = 29_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(19) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int__int_  = { let mut __tmp_369 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int__int_ :: new ( { & mut __tmp_369 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_370 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_370 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_371 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_371 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_372 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_372 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_373 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_373 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_374 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_374 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_375 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_375 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_376 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_376 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_377 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_377 } , )  } ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_378 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_378 } , )  } ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_379 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_379 } , )  } ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_380 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_380 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_381 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_381 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_382 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_382 } , )  } ;
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
    let mut __tmp_383: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_383;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_384 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_384 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_385 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_385 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_386 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_386 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_387 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_387 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_388 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_388 } , )  } ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_389 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_389 } , )  } ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_390 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_390 } , )  } ;
        let mut _args_3: () = 10_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_391 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_391 } , )  } ;
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
    let mut __tmp_392: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_392;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_  = { let mut __tmp_393 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ :: new ( { & mut __tmp_393 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref_ ,*mut () ,)>(47) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_  = { let mut __tmp_394 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ :: new ( { & mut __tmp_394 } , )  } ;
        let mut _args_3: () = 29_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int_ ,*mut () ,*mut () ,)>(48) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int__int_  = { let mut __tmp_395 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_double__double__std_less_double___std_allocator_std_pair_const_double__double____ref__int__int_ :: new ( { & mut __tmp_395 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_396 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_396 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_397 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_397 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_398 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_398 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_399 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_399 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_400 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_400 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_401 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_401 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_402 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_402 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_403 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_403 } , )  } ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_404 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_404 } , )  } ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_405 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_405 } , )  } ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_406 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_406 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_407 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_407 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_408 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_408 } , )  } ;
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
    let mut __tmp_409: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_409;
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
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_410 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_410 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_411 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_411 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_412 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_412 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_413 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_413 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_414 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_414 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_415 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_415 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_416 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_416 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_417 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_417 } , )  } ;
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
    let mut __tmp_418: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_418;
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
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_419 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_419 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(21) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_420 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_420 } , )  } ;
        let mut _args_3: () = 18_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(22) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int__int_  = { let mut __tmp_421 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int__int_ :: new ( { & mut __tmp_421 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_422 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_422 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_423 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_423 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_424 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_424 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_425 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_425 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_426 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_426 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_427 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_427 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_428 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_428 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_429 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_429 } , )  } ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_430 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_430 } , )  } ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_431 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_431 } , )  } ;
        let mut _args_3: () = 15_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_432 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_432 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_433 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_433 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_434 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_434 } , )  } ;
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
    let mut __tmp_435: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_435;
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
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_436 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_436 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_437 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_437 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_438 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_438 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_439 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_439 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_440 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_440 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_441 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_441 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_442 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_442 } , )  } ;
        let mut _args_3: () = 13_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_443 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_443 } , )  } ;
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
    let mut __tmp_444: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_444;
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
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_  = { let mut __tmp_445 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ :: new ( { & mut __tmp_445 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref_ ,*mut () ,)>(50) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_  = { let mut __tmp_446 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ :: new ( { & mut __tmp_446 } , )  } ;
        let mut _args_3: () = 18_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int_ ,*mut () ,*mut () ,)>(51) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int__int_  = { let mut __tmp_447 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_map_const_void_ptr__std_nullptr_t__std_hash_const_void_ptr___std_equal_to_const_void_ptr___std_allocator_std_pair_const_void_ptrconst__std_nullptr_t____ref__int__int_ :: new ( { & mut __tmp_447 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_448 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_448 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_449 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_449 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_450 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_450 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_451 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_451 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_452 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_452 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_453 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_453 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_454 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_454 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = { let mut __tmp_455 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ :: new ( { & mut __tmp_455 } , )  } ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = { let mut __tmp_456 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ :: new ( { & mut __tmp_456 } , )  } ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = { let mut __tmp_457 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ :: new ( { & mut __tmp_457 } , )  } ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = { let mut __tmp_458 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ :: new ( { & mut __tmp_458 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_459 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_459 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_460 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_460 } , )  } ;
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
    let mut __tmp_461: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_461;
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
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_462 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_462 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_463 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_463 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_464 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_464 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_465 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_465 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = { let mut __tmp_466 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ :: new ( { & mut __tmp_466 } , )  } ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = { let mut __tmp_467 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ :: new ( { & mut __tmp_467 } , )  } ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = { let mut __tmp_468 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ :: new ( { & mut __tmp_468 } , )  } ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = { let mut __tmp_469 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ :: new ( { & mut __tmp_469 } , )  } ;
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
    let mut __tmp_470: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_470;
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
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_  = { let mut __tmp_471 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ :: new ( { & mut __tmp_471 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref_ ,*mut () ,)>(24) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_  = { let mut __tmp_472 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ :: new ( { & mut __tmp_472 } , )  } ;
        let mut _args_3: () = 51_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int_ ,*mut () ,*mut () ,)>(25) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int__int_  = { let mut __tmp_473 : ()  = unimplemented!() ; std_basic_format_string_char__std_map_std_string__std_string__std_less_std_string___std_allocator_std_pair_const_std_string__std_string____ref__int__int_ :: new ( { & mut __tmp_473 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_474 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_474 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_475 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_475 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_476 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_476 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_477 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_477 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_478 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_478 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_479 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_479 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_480 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_480 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = { let mut __tmp_481 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ :: new ( { & mut __tmp_481 } , )  } ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = { let mut __tmp_482 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ :: new ( { & mut __tmp_482 } , )  } ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = { let mut __tmp_483 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ :: new ( { & mut __tmp_483 } , )  } ;
        let mut _args_3: () = 41_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = { let mut __tmp_484 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ :: new ( { & mut __tmp_484 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_485 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_485 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_486 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_486 } , )  } ;
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
    let mut __tmp_487: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_487;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_488 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_488 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_489 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_489 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_490 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_490 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_491 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_491 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = { let mut __tmp_492 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ :: new ( { & mut __tmp_492 } , )  } ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = { let mut __tmp_493 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ :: new ( { & mut __tmp_493 } , )  } ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = { let mut __tmp_494 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ :: new ( { & mut __tmp_494 } , )  } ;
        let mut _args_3: () = 21_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = { let mut __tmp_495 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ :: new ( { & mut __tmp_495 } , )  } ;
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
    let mut __tmp_496: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_496;
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
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_  = { let mut __tmp_497 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ :: new ( { & mut __tmp_497 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref_ ,*mut () ,)>(53) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_  = { let mut __tmp_498 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ :: new ( { & mut __tmp_498 } , )  } ;
        let mut _args_3: () = 51_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int_ ,*mut () ,*mut () ,)>(54) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int__int_  = { let mut __tmp_499 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_map_std_wstring__std_wstring__std_less_std_wstring___std_allocator_std_pair_const_std_wstring__std_wstring____ref__int__int_ :: new ( { & mut __tmp_499 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_500 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_500 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_501 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_501 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_502 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_502 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_503 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_503 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_504 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_504 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_505 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_505 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_506 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_506 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = { let mut __tmp_507 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ :: new ( { & mut __tmp_507 } , )  } ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(28) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = { let mut __tmp_508 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ :: new ( { & mut __tmp_508 } , )  } ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(28) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = { let mut __tmp_509 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ :: new ( { & mut __tmp_509 } , )  } ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(28) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = { let mut __tmp_510 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ :: new ( { & mut __tmp_510 } , )  } ;
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
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_511 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_511 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(27) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_512 : ()  = unimplemented!() ; std_basic_format_string_char__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_512 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_513 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_513 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_514 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_514 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_515 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_515 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_516 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_516 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_517 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_517 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_518 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_518 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_519 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_519 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = { let mut __tmp_520 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ :: new ( { & mut __tmp_520 } , )  } ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(57) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = { let mut __tmp_521 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ :: new ( { & mut __tmp_521 } , )  } ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(57) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = { let mut __tmp_522 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ :: new ( { & mut __tmp_522 } , )  } ;
        let mut _args_3: () = 37_();
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ ,*mut () ,*mut () ,)>(57) .call ( _expected , _fmt , & mut input , & mut _args_3 , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_  = { let mut __tmp_523 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref__int_ :: new ( { & mut __tmp_523 } , )  } ;
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
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_524 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_524 } , )  } ;
        check  .spec::<fn(() ,std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ ,*mut () ,)>(56) .call ( _expected , _fmt , & mut input , )
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_  = { let mut __tmp_525 : ()  = unimplemented!() ; std_basic_format_string_wchar_t__std_unordered_multimap_status__status__std_hash_status___std_equal_to_status___std_allocator_std_pair_const_status__status____ref_ :: new ( { & mut __tmp_525 } , )  } ;
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
            let mut __tmp_526: () = unimplemented!();
            adaptor::new({ &mut __tmp_526 })
        };
        test_int_37(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_adaptor_52(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe {
        let mut _input: adaptor = {
            let mut __tmp_527: () = unimplemented!();
            adaptor::new({ &mut __tmp_527 })
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
        (unsafe { test_transcode_69(b.clone(), e.clone(), unimplemented!()) });
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
