extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_eprintf_0(
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
pub unsafe fn test_log_1(
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
        test_eprintf_0(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
pub unsafe fn test_is_high_surrogate_2(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (56319_()));
}
pub unsafe fn test_is_low_surrogate_3(mut value: ()) -> () {
    return ((value) >= (56320_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_surrogate_4(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_code_point_5(mut value: ()) -> () {
    return ((value) <= (1114111_()));
}
pub unsafe fn test_is_scalar_value_6(mut value: ()) -> () {
    return (unsafe { test_is_code_point_5(value) }) && (!(unsafe { test_is_surrogate_4(value) }));
}
pub static mut test_replacement_character_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (65533 as ()) });
pub unsafe fn operator_shl_8(os: *mut (), loc: *const ()) -> *mut () {
    return &mut (*unimplemented!());
}
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
pub unsafe fn operator_cmp_9(
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
            operator_cmp_9(
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
            operator_cmp_9(
                self as *const parse_call_validator,
                other as *const parse_call_validator,
            ) == std::cmp::Ordering::Equal
        }
    }
}
impl std::cmp::Eq for parse_call_validator {}
pub unsafe fn get_format_types_10() -> () {
    return unimplemented!();
}
pub unsafe fn test_exception_11(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_12(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_13(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_14(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_15(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_16(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_17(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_18(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_19(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_20(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_21(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_22(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_23(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_24(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_25(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_26(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_27(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_28(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_29(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_30(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_31(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_32(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_33(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_34(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_35(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_36(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_37(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_38(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_39(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_exception_40(mut fmt: (), args_1: *mut (), args_2: *mut ()) {
    let mut what: () = unimplemented!();
}
pub unsafe fn test_41() {
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = true;
        test_exception_11(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = ('0' as ());
        test_exception_12(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    {}
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = 0_();
        test_exception_13(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = 0_();
        test_exception_14(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = 42_f32;
        test_exception_15(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = 42_f64;
        test_exception_16(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = 42_f64;
        test_exception_17(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = true;
        test_exception_18(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = ('0' as ());
        test_exception_19(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    {}
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = 0_();
        test_exception_20(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = 0_();
        test_exception_21(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = 42_f32;
        test_exception_22(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = 42_f64;
        test_exception_23(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = 42_f64;
        test_exception_24(unimplemented!(), &mut _args_1, &mut _args_2)
    });
}
pub unsafe fn test_42() {
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = true;
        test_exception_25(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = ('0' as ());
        test_exception_26(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    {
        (unsafe {
            let mut _args_1: () = 42_();
            let mut _args_2: () = (48 as ());
            test_exception_27(unimplemented!(), &mut _args_1, &mut _args_2)
        });
    }
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = 0_();
        test_exception_28(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = 0_();
        test_exception_29(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = 42_f32;
        test_exception_30(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = 42_f64;
        test_exception_31(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_();
        let mut _args_2: () = 42_f64;
        test_exception_32(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = true;
        test_exception_33(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = ('0' as ());
        test_exception_34(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    {
        (unsafe {
            let mut _args_1: () = 42_f64;
            let mut _args_2: () = (48 as ());
            test_exception_35(unimplemented!(), &mut _args_1, &mut _args_2)
        });
    }
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = 0_();
        test_exception_36(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = 0_();
        test_exception_37(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = 42_f32;
        test_exception_38(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = 42_f64;
        test_exception_39(unimplemented!(), &mut _args_1, &mut _args_2)
    });
    (unsafe {
        let mut _args_1: () = 42_f64;
        let mut _args_2: () = 42_f64;
        test_exception_40(unimplemented!(), &mut _args_1, &mut _args_2)
    });
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
    (unsafe { test_41() });
    (unsafe { test_42() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_7);
}
