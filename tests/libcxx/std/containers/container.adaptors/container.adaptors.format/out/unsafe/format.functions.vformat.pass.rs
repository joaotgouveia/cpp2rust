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
    return (unsafe { fmt_invalid_types_10(unimplemented!()) });
}
pub unsafe fn fmt_invalid_types_16(mut valid: ()) -> () {
    return (unsafe { fmt_invalid_types_13(unimplemented!()) });
}
pub unsafe fn fmt_invalid_nested_types_17(mut valid: ()) -> () {
    return (unsafe { fmt_invalid_types_12(unimplemented!()) });
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
pub unsafe fn test_char_default_26(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_0: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_0;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_1: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_1;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check.spec::<fn((), (), *mut (), *mut (), *mut ())>(2).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
            &mut _args_4,
        )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_27(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_2: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_2;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(2)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_3: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_3;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(2)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check.spec::<fn((), (), *mut (), *mut (), *mut ())>(5).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
            &mut _args_4,
        )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(3)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_28(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_4: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_4;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(4)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_5: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_5;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(4)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check.spec::<fn((), (), *mut (), *mut (), *mut ())>(8).call(
            _expected,
            _fmt,
            _args_2,
            &mut _args_3,
            &mut _args_4,
        )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(5)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_29(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_6: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_6;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(6)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_7: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_7;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(6)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(9)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(10)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(11)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(6)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_30(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_8: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_8;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(8)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_9: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_9;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(8)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(12)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(13)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(14)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(8)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(9)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_31(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_10: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_10;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_11: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_11;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(10)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(15)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(17)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(10)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(11)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_32(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_12: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_12;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(15)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_13: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_13;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(15)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(18)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(19)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(20)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(15)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(16)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_33(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_14: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_14;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(17)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_15: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_15;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(17)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(21)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(22)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(23)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(17)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(18)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_34(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_16: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_16;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(19)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_17: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_17;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(19)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(24)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(25)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(26)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(19)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(20)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_35(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_18: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_18;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(57)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_19: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_19;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(57)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(83)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_36(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_20: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_20;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(59)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_21: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_21;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(59)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(86)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(60)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_37(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_22: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_22;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(61)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_23: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_23;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(61)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(89)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(62)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_38(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(91)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(91)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(91)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(91)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_24: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_24;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(63)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(91)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(91)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(91)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(91)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_25: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_25;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(63)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(90)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(91)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(92)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(63)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(64)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_39(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(94)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(94)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(94)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(94)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_26: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_26;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(65)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(94)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(94)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(94)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(94)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_27: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_27;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(65)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(93)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(94)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(95)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(65)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(66)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_default_40(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(97)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(97)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(97)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 30_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(97)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_28: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_28;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(67)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(97)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(97)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(97)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 4_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(97)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_29: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_29;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(67)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(96)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(97)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(98)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(67)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(68)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_char_string_41(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_30: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_30;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(0)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_char_string_42(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_31: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_31;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(2)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_char_string_43(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_32: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_32;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(4)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_char_string_44(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_33: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_33;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(57)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_char_string_45(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_34: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_34;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(59)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_char_string_46(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_35: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_35;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(61)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_char_escaped_string_47(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(1)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(0)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_char_escaped_string_48(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(3)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(4)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(2)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_char_escaped_string_49(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(6)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(7)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(4)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_char_escaped_string_50(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(81)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(57)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_char_escaped_string_51(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(84)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(59)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_char_escaped_string_52(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(87)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(61)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_char_53(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_26(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_27(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_28(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_string_41(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_string_42(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_string_43(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_escaped_string_47(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_escaped_string_48(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_escaped_string_49(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_29(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_30(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_31(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_char_54(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_35(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_36(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_37(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_string_44(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_string_45(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_string_46(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_escaped_string_50(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_escaped_string_51(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_escaped_string_52(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_38(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_39(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_40(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_char_to_wchar_55(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_32(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_33(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_char_default_34(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _args: () = unimplemented!();
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut _args,
        )
    });
    (unsafe {
        let mut _args: () = unimplemented!();
        check_exception.spec::<fn((), (), *mut ())>(13).call(
            unimplemented!(),
            unimplemented!(),
            &mut _args,
        )
    });
    (unsafe {
        let mut _args: () = unimplemented!();
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut _args,
        )
    });
    (unsafe {
        let mut _args: () = unimplemented!();
        check_exception.spec::<fn((), (), *mut ())>(12).call(
            unimplemented!(),
            unimplemented!(),
            &mut _args,
        )
    });
    (unsafe {
        let mut _args: () = unimplemented!();
        check_exception.spec::<fn((), (), *mut ())>(13).call(
            unimplemented!(),
            unimplemented!(),
            &mut _args,
        )
    });
    (unsafe {
        let mut _args: () = unimplemented!();
        check_exception.spec::<fn((), (), *mut ())>(14).call(
            unimplemented!(),
            unimplemented!(),
            &mut _args,
        )
    });
}
pub unsafe fn test_bool_56(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_36: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_36;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(21)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_37: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_37;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(21)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(27)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(28)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        let mut _args_4: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(29)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(21)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(22)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_bool_57(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_38: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_38;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(23)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_39: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_39;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(23)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(30)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(31)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        let mut _args_4: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(32)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(23)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(24)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_bool_58(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_40: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_40;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(25)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_41: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_41;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(25)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(33)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        let mut _args_4: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(35)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(25)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(26)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_bool_59(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(100)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(100)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(100)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(100)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_42: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_42;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(69)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(100)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(100)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(100)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(100)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_43: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_43;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(69)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(99)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(100)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        let mut _args_4: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(101)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(69)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(70)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_bool_60(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(103)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(103)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(103)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(103)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_44: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_44;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(71)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(103)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(103)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(103)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(103)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_45: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_45;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(71)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(102)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(103)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        let mut _args_4: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(104)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(71)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(72)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_bool_61(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(106)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(106)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(106)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 24_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(106)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_46: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_46;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(73)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(106)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(106)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(106)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(106)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_47: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_47;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(73)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(105)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(106)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        let mut _args_4: () = 7_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(107)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(73)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 32_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(74)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_bool_62(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_bool_56(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_bool_57(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_bool_58(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_bool_63(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_bool_59(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_bool_60(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_bool_61(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_int_64(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_48: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_48;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(27)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_49: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_49;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(27)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(36)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(38)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(27)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(28)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_int_65(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_50: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_50;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(29)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_51: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_51;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(29)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(39)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(41)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(29)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(30)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_int_66(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_52: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_52;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(31)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_53: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_53;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(31)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(42)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(44)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(31)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(32)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_int_67(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(109)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(109)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(109)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(109)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_54: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_54;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(75)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(109)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(109)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(109)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(109)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_55: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_55;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(75)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(108)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(109)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(110)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(75)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_int_68(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(112)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(112)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(112)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(112)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_56: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_56;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(77)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(112)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(112)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(112)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(112)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_57: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_57;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(77)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(111)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(112)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(113)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(77)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(78)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_int_69(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(115)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(115)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(115)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 20_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(115)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_58: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_58;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(79)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(115)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(115)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(115)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(115)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_59: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_59;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(79)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(114)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(115)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(116)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(79)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(80)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_int_70(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_int_64(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_int_65(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_int_66(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_int_71(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_int_67(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_int_68(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_int_69(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_floating_point_72(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_60: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_60;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(33)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_61: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_61;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(33)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(47)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(45)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(46)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(47)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        let mut _args_5: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut (), *mut ())>(48)
            .call(
                _expected,
                _fmt,
                _args_2,
                &mut _args_3,
                &mut _args_4,
                &mut _args_5,
            )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(33)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(34)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check_exception
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(35)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
}
pub unsafe fn test_floating_point_73(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_62: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_62;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(36)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_63: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_63;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(36)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(51)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(49)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(50)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(51)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        let mut _args_5: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut (), *mut ())>(52)
            .call(
                _expected,
                _fmt,
                _args_2,
                &mut _args_3,
                &mut _args_4,
                &mut _args_5,
            )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(36)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(37)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check_exception
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(38)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
}
pub unsafe fn test_floating_point_74(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(54)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(54)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(54)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(54)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_64: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_64;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(39)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(54)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(54)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(54)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(54)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(54)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(54)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_65: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_65;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(39)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(54)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(53)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(54)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(55)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        let mut _args_5: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut (), *mut ())>(56)
            .call(
                _expected,
                _fmt,
                _args_2,
                &mut _args_3,
                &mut _args_4,
                &mut _args_5,
            )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(39)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(40)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check_exception
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(41)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
}
pub unsafe fn test_floating_point_75(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(118)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(118)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(118)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(118)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_66: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_66;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(81)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(118)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(118)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(118)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(118)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(118)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(118)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_67: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_67;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(81)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(118)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(119)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(117)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(118)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(119)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        let mut _args_5: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut (), *mut ())>(120)
            .call(
                _expected,
                _fmt,
                _args_2,
                &mut _args_3,
                &mut _args_4,
                &mut _args_5,
            )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(81)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(82)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check_exception
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(83)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
}
pub unsafe fn test_floating_point_76(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(122)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(122)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(122)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(122)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_68: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_68;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(84)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(122)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(122)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(122)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(122)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(122)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(122)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_69: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_69;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(84)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(122)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(123)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(121)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(122)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(123)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        let mut _args_5: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut (), *mut ())>(124)
            .call(
                _expected,
                _fmt,
                _args_2,
                &mut _args_3,
                &mut _args_4,
                &mut _args_5,
            )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(84)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(85)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check_exception
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(86)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
}
pub unsafe fn test_floating_point_77(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(126)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(126)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(126)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 27_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(126)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_70: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_70;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(87)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(126)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(126)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(126)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(126)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(126)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(126)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    let mut __tmp_71: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_71;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(87)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(126)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(127)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(125)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(126)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(127)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        let mut _args_5: () = 2_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut (), *mut ())>(128)
            .call(
                _expected,
                _fmt,
                _args_2,
                &mut _args_3,
                &mut _args_4,
                &mut _args_5,
            )
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(87)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(88)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 33_();
        let mut _args_4: () = 5_();
        check_exception
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(89)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
}
pub unsafe fn test_floating_point_78(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_floating_point_72(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_floating_point_73(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_floating_point_74(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_floating_point_79(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_floating_point_75(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_floating_point_76(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_floating_point_77(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_pointer_80(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_72: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_72;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(42)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_73: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_73;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(42)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(59)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(57)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(58)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(59)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(42)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(43)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_pointer_81(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_74: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_74;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(44)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_75: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_75;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(44)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(62)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(60)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(61)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(62)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(44)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(45)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_pointer_82(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_76: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_76;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(46)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_77: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_77;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(46)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(65)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(63)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(64)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(65)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(46)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(47)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_pointer_83(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(130)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(130)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(130)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(130)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_78: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_78;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(90)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(130)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(130)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(130)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(130)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_79: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_79;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(90)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(130)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(131)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(129)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(130)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(131)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(90)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(91)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_pointer_84(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(133)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(133)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(133)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(133)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_80: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_80;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(92)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(133)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(133)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(133)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(133)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_81: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_81;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(92)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(133)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(134)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(132)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(133)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(134)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(92)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(93)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_pointer_85(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(136)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(136)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(136)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 10_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(136)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_82: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_82;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(94)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(136)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(136)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(136)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(136)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_83: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_83;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(94)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(136)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(137)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(135)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(136)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        let mut _args_4: () = 5_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(137)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(94)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 12_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(95)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_pointer_86(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_pointer_80(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_pointer_81(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_pointer_82(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_pointer_87(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_pointer_83(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_pointer_84(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_pointer_85(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_string_88(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_84: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_84;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(48)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_85: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_85;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(48)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(68)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(66)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(67)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(68)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(48)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(49)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_string_89(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_86: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_86;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(50)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_87: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_87;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(50)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(71)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(69)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(70)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(71)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(50)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(51)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_string_90(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_88: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_88;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(52)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_89: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_89;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(52)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(74)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(72)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(73)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(74)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(52)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(53)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_string_91(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(139)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(139)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(139)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(139)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_90: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_90;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(96)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(139)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(139)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(139)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(139)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(139)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_91: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_91;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(96)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(139)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(140)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(138)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(139)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(140)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(96)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(97)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_string_92(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(142)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(142)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(142)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(142)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_92: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_92;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(98)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(142)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(142)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(142)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(142)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(142)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_93: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_93;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(98)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(142)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(143)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(141)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(142)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(143)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(98)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(99)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_string_93(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(145)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(145)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(145)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 23_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(145)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_94: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_94;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(100)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(145)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(145)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(145)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(145)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 3_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(145)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_95: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_95;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(100)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(145)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(146)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(144)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(145)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        let mut _args_4: () = 8_();
        check
            .spec::<fn((), (), *mut (), *mut (), *mut ())>(146)
            .call(_expected, _fmt, _args_2, &mut _args_3, &mut _args_4)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(100)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 25_();
        check_exception
            .spec::<fn((), (), *mut (), *mut ())>(101)
            .call(unimplemented!(), _fmt, _args_2, &mut _args_3)
    });
}
pub unsafe fn test_string_94(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_string_88(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_string_89(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_string_90(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_string_95(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_string_91(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_string_92(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_string_93(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_status_96(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_96: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_96;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(54)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_97: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_97;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(54)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(75)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(76)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(54)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_status_97(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(77)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(77)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(77)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(77)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(77)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(77)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(77)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(78)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(78)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(78)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(78)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(77)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_98: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_98;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(55)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_99: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_99;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(55)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(77)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(77)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(77)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(77)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(78)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(55)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_status_98(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(79)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(79)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(79)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(79)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(79)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(79)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(79)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(80)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(80)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(80)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(80)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(79)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_100: () = (unsafe { fmt_invalid_types_15(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_100;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(56)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_101: () = (unsafe { fmt_invalid_nested_types_17(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_101;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(56)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(79)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(79)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(79)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(79)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(80)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(56)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_status_99(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(147)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(147)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(147)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(147)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(147)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(147)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(147)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(148)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(148)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(148)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(148)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(147)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_102: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_102;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(102)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_103: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_103;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(102)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(147)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(147)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(147)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(147)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(148)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(102)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_status_100(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(149)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(149)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(149)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(149)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(149)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(149)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(149)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(150)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(150)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(150)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(150)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(149)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_104: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_104;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(103)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_105: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_105;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(103)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(149)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(149)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(149)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(149)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(150)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(103)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_status_101(
    mut check: FnPtr<Generic>,
    mut check_exception: FnPtr<Generic>,
    input: *mut (),
) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(151)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(151)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(151)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(151)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(151)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(151)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(151)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(152)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(152)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(152)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(152)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(151)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_106: () = (unsafe { fmt_invalid_types_16(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_106;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(104)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
    let mut __tmp_107: () = (unsafe { fmt_invalid_nested_types_18(unimplemented!()) });
    let __range1: *mut () = &mut __tmp_107;
    'loop_: for fmt in IterRange::new(unimplemented!(), unimplemented!()) {
        let mut fmt: () = (unsafe { (*unimplemented!())() });
        (unsafe {
            let _fmt: () = unimplemented!();
            let _args: *mut () = input;
            check_exception
                .spec::<fn((), (), *mut ())>(104)
                .call(unimplemented!(), _fmt, _args)
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(151)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(151)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(151)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check
            .spec::<fn((), (), *mut ())>(151)
            .call(_expected, _fmt, _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let _args_2: *mut () = input;
        let mut _args_3: () = 29_();
        check
            .spec::<fn((), (), *mut (), *mut ())>(152)
            .call(_expected, _fmt, _args_2, &mut _args_3)
    });
    (unsafe {
        let _fmt: () = unimplemented!();
        let _args: *mut () = input;
        check_exception
            .spec::<fn((), (), *mut ())>(104)
            .call(unimplemented!(), _fmt, _args)
    });
}
pub unsafe fn test_status_102(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_status_96(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_status_97(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_status_98(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn test_status_103(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    let mut input: () = unimplemented!();
    (unsafe {
        let mut _input: () = unimplemented!();
        test_status_99(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_status_100(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
    (unsafe {
        let mut _input: () = unimplemented!();
        test_status_101(check.copy_from(), check_exception.copy_from(), &mut _input)
    });
}
pub unsafe fn format_tests_104(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe { test_char_53(check.copy_from(), check_exception.copy_from()) });
    {}
    (unsafe { test_bool_62(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_int_70(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_floating_point_78(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_pointer_86(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_string_94(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_status_102(check.copy_from(), check_exception.copy_from()) });
}
pub unsafe fn format_tests_105(mut check: FnPtr<Generic>, mut check_exception: FnPtr<Generic>) {
    (unsafe { test_char_54(check.copy_from(), check_exception.copy_from()) });
    {
        (unsafe { test_char_to_wchar_55(check.copy_from(), check_exception.copy_from()) });
    }
    (unsafe { test_bool_63(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_int_71(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_floating_point_79(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_pointer_87(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_string_95(check.copy_from(), check_exception.copy_from()) });
    (unsafe { test_status_103(check.copy_from(), check_exception.copy_from()) });
}
pub unsafe fn test_eprintf_106(
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
pub unsafe fn test_eprintf_107(
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
pub unsafe fn test_eprintf_108(mut fmt: *const (), args: *const *const ()) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args),
        )
    });
}
pub unsafe fn test_log_109(
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
        test_eprintf_106(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
pub unsafe fn test_log_110(
    mut condition: *const (),
    mut file: *const (),
    mut line: (),
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_107(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_require_111(
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
        test_log_110(_condition, _file, _line, _functor)
    });
    unimplemented!();
}
pub unsafe fn test_is_high_surrogate_112(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (56319_()));
}
pub unsafe fn test_is_low_surrogate_113(mut value: ()) -> () {
    return ((value) >= (56320_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_surrogate_114(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_code_point_115(mut value: ()) -> () {
    return ((value) <= (1114111_()));
}
pub unsafe fn test_is_scalar_value_116(mut value: ()) -> () {
    return (unsafe { test_is_code_point_115(value) })
        && (!(unsafe { test_is_surrogate_114(value) }));
}
pub static mut test_replacement_character_117: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (65533 as ()) });
pub unsafe fn test_encode_118(out_it: *mut (), mut value: ()) {
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
pub unsafe fn test_encode_119(out_it: *mut (), mut value: ()) {
    if (((value) & (4294901760_())) == (0_())) {
        (unsafe {
            let _out_it: *mut () = out_it;
            let _value: () = value;
            test_encode_118(_out_it, _value)
        });
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_transcode_120(mut first: (), mut last: (), mut out_it: ()) -> () {
    'loop_: while !(unimplemented!()) {
        let mut value: () = (*unimplemented!());
        if (unsafe { test_is_code_point_115(value) }) {
            (unsafe { test_encode_119(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_118(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(
                        &mut *&raw mut test_replacement_character_117,
                    )),
                )
            });
        }
    }
    return out_it;
}
pub unsafe fn test_transcode_121(mut first: *const (), mut last: *const (), mut out_it: ()) -> () {
    'loop_: while ((first) != (last)) {
        let mut value: () = (*first.postfix_inc());
        if (unsafe { test_is_code_point_115(value) }) {
            (unsafe { test_encode_119(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_118(
                    &mut out_it,
                    (*std::cell::LazyCell::force_mut(
                        &mut *&raw mut test_replacement_character_117,
                    )),
                )
            });
        }
    }
    return out_it;
}
pub unsafe fn operator_shl_122(os: *mut (), loc: *const ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_123(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_124(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_125(stream: *mut (), value: *const [(); 18]) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_126(stream: *mut (), value: *mut ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_127(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if !(unimplemented!()) {
        {}
        (unsafe { test_transcode_120(unimplemented!(), unimplemented!(), unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_128(stream: *mut (), value: *mut ()) -> *mut () {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ((b) != (e)) {
        {}
        (unsafe { test_transcode_121(b, e, unimplemented!()) });
    }
    return stream;
}
pub unsafe fn test_concat_message_129(
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
                test_concat_125(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_126(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_125(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_126(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_125(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_124(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
                                    test_concat_123(_stream, _value)
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
pub unsafe fn test_concat_message_130(
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
                test_concat_125(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut () = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_128(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut () = &mut sstr;
                        let _value: *const [(); 18] = args_2;
                        test_concat_125(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut () = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_128(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut () = &mut sstr;
                                let _value: *const [(); 18] = args_4;
                                test_concat_125(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_5;
                                    test_concat_127(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut () = &mut sstr;
                                    let _value: *mut () = args_6;
                                    test_concat_123(_stream, _value)
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
pub static mut test_131: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_111(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_132(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_133(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_134(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_135(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_136(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_137(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_138(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_139(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_140(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_141(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_142(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_143(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_144(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_145(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_146(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_147(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_148(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_149(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_150(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_151(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_152(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_153(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_154(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_155(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_156(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_157(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_158(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_159(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_160(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_161(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_162(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_163(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_164(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_165(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_166(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_167(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_168(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_169(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_170(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_171(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_172(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_173(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_174(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_175(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_176(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_177(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_178(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (),
             mut fmt: (),
             args_2: *mut (),
             args_3: *mut (),
             args_4: *mut (),
             args_5: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_179(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_180(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_181(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_182(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (),
             mut fmt: (),
             args_2: *mut (),
             args_3: *mut (),
             args_4: *mut (),
             args_5: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_183(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_184(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_185(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_186(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (),
             mut fmt: (),
             args_2: *mut (),
             args_3: *mut (),
             args_4: *mut (),
             args_5: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_187(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_188(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_189(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_190(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_191(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_192(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_193(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_194(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_195(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_196(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_197(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_198(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_199(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_200(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_201(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_202(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_203(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_204(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_205(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_206(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_207(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_208(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_209(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_210(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_211(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_212(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_213(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_214(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_215(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_216(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_217(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_218(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_219(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_220(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_221(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_222(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_223(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_224(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_225(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_226(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_227(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_228(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_229(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_230(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_231(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_232(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_233(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_234(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_235(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_236(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_237(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_238(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_239(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_240(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_241(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_242(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_243(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_244(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_245(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_246(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_247(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_248(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_249(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_250(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (),
             mut fmt: (),
             args_2: *mut (),
             args_3: *mut (),
             args_4: *mut (),
             args_5: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_251(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_252(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_253(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_254(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (),
             mut fmt: (),
             args_2: *mut (),
             args_3: *mut (),
             args_4: *mut (),
             args_5: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_255(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_256(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_257(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_258(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (),
             mut fmt: (),
             args_2: *mut (),
             args_3: *mut (),
             args_4: *mut (),
             args_5: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_259(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_260(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_261(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_262(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_263(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_264(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_265(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_266(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_267(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_268(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_269(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_270(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_271(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_272(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_273(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_274(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_275(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_276(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_277(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_278(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_279(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_280(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_281(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_282(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            },
            |mut expected: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {
                let mut out: () = unimplemented!();
                (unsafe {
                    let _condition: () = unimplemented!();
                    let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                        {
                            let fmt: *mut () = &mut fmt;
                            let expected: *mut () = &mut expected;
                            let out: *mut () = &mut out;
                        },
                        || {
                            (unsafe {
                                let mut _args: *const () = (unimplemented!() as *const ());
                                test_eprintf_108(c"%s".as_ptr(), &mut _args)
                            });
                        }
                    );
                    test_require_283(
                        _condition,
                        c"out == expected".as_ptr(),
                        c"format.functions.vformat.pass.cpp".as_ptr(),
                        39_(),
                        &mut _arg,
                    )
                });
            }
        )
    });
pub static mut test_exception_284: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut (), args_4: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args_2: *mut (), args_3: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {},
            |mut what: (), mut fmt: (), args: *mut ()| {}
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
        format_tests_104(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_131)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_284)).copy_from(),
        )
    });
    (unsafe {
        format_tests_105(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_131)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_284)).copy_from(),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_117);
    std::cell::LazyCell::force(&*&raw const test_131);
    std::cell::LazyCell::force(&*&raw const test_exception_284);
}
