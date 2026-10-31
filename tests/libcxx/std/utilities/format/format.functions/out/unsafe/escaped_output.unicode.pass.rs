extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_into(*const libc::c_char , to_pconstlibcc_char)]
#[cpp_into(*const i32 , to_pconsti32)]
#[cpp_into(*const u8 , to_pconstu8)]
#[cpp_into(*const u16 , to_pconstu16)]
#[cpp_into(*const u32 , to_pconstu32)]
pub struct MultiStringType {
    pub w_: *const i32,
    pub wn_: usize,
    pub u8_: *const u8,
    pub u8n_: usize,
    pub u16_: *const u16,
    pub u16n_: usize,
    pub u32_: *const u32,
    pub u32n_: usize,
    pub s_: *const libc::c_char,
    pub sn_: usize,
}
impl MultiStringType {
    pub unsafe fn new(
        mut w: *const i32,
        mut wn: usize,
        mut u8: *const u8,
        mut u8n: usize,
        mut u16: *const u16,
        mut u16n: usize,
        mut u32: *const u32,
        mut u32n: usize,
        mut s: *const libc::c_char,
        mut sn: usize,
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
    pub unsafe fn as_ptr_2(&self, mut _a0: *const libc::c_char) -> *const libc::c_char {
        return self.s_;
    }
    pub unsafe fn as_ptr_3(&self, mut _a0: *const i32) -> *const i32 {
        return self.w_;
    }
    pub unsafe fn as_ptr_4(&self, mut _a0: *const u8) -> *const u8 {
        return self.u8_;
    }
    pub unsafe fn as_ptr_5(&self, mut _a0: *const u16) -> *const u16 {
        return self.u16_;
    }
    pub unsafe fn as_ptr_6(&self, mut _a0: *const u32) -> *const u32 {
        return self.u32_;
    }
    pub unsafe fn length_7(&self, mut _a0: *const libc::c_char) -> usize {
        return self.sn_;
    }
    pub unsafe fn length_8(&self, mut _a0: *const i32) -> usize {
        return self.wn_;
    }
    pub unsafe fn length_9(&self, mut _a0: *const u8) -> usize {
        return self.u8n_;
    }
    pub unsafe fn length_10(&self, mut _a0: *const u16) -> usize {
        return self.u16n_;
    }
    pub unsafe fn length_11(&self, mut _a0: *const u32) -> usize {
        return self.u32n_;
    }
    pub unsafe fn to_pconstlibcc_char(&self) -> *const libc::c_char {
        return self.s_;
    }
    pub unsafe fn to_pconsti32(&self) -> *const i32 {
        return self.w_;
    }
    pub unsafe fn to_pconstu8(&self) -> *const u8 {
        return self.u8_;
    }
    pub unsafe fn to_pconstu16(&self) -> *const u16 {
        return self.u16_;
    }
    pub unsafe fn to_pconstu32(&self) -> *const u32 {
        return self.u32_;
    }
}
pub unsafe fn test_eprintf_0(
    mut fmt: *const libc::c_char,
    args_1: *const *const libc::c_char,
    args_2: *const *const libc::c_char,
    args_3: *const *const libc::c_char,
    args_4: *const i32,
    args_5: *const *const libc::c_char,
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
pub unsafe fn test_eprintf_1(
    mut fmt: *const libc::c_char,
    args_1: *const *const libc::c_char,
    args_2: *const *const libc::c_char,
    args_3: *const i32,
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
pub unsafe fn test_eprintf_2(mut fmt: *const libc::c_char, args: *const *const libc::c_char) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args),
        )
    });
}
pub unsafe fn test_log_3(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    mut message: *const libc::c_char,
) {
    let mut msg: *const libc::c_char = if !(condition).is_null() {
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
pub unsafe fn test_log_4(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_5(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_6(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_7(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_8(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_9(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_10(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_11(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_12(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_13(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_14(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_15(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_16(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_17(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_log_18(
    mut condition: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    functor: *const FnPtr<fn()>,
) {
    (unsafe {
        test_eprintf_1(
            c"Assertion failure: %s %s %d\n".as_ptr(),
            &condition,
            &file,
            &line,
        )
    });
    (unsafe { (*functor).call() });
}
pub unsafe fn test_require_19(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_4(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_20(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_5(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_21(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_6(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_22(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_7(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_23(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_8(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_24(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_9(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_25(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_10(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_26(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_11(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_27(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_12(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_28(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_13(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_29(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_14(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_30(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_15(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_31(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_16(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_32(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_17(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_require_33(
    mut condition: bool,
    mut condition_str: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: i32,
    arg: *const FnPtr<fn()>,
) {
    if condition {
        return;
    }
    (unsafe {
        let _condition: *const libc::c_char = condition_str;
        let _file: *const libc::c_char = file;
        let _line: i32 = line;
        let _functor: *const FnPtr<fn()> = arg;
        test_log_18(_condition, _file, _line, _functor)
    });
    std::process::abort();
}
pub unsafe fn test_is_high_surrogate_34(mut value: u32) -> bool {
    return ((value) >= (55296_u32)) && ((value) <= (56319_u32));
}
pub unsafe fn test_is_low_surrogate_35(mut value: u32) -> bool {
    return ((value) >= (56320_u32)) && ((value) <= (57343_u32));
}
pub unsafe fn test_is_surrogate_36(mut value: u32) -> bool {
    return ((value) >= (55296_u32)) && ((value) <= (57343_u32));
}
pub unsafe fn test_is_code_point_37(mut value: u32) -> bool {
    return ((value) <= (1114111_u32));
}
pub unsafe fn test_is_scalar_value_38(mut value: u32) -> bool {
    return (unsafe { test_is_code_point_37(value) })
        && (!(unsafe { test_is_surrogate_36(value) }));
}
pub static mut test_replacement_character_39: std::cell::LazyCell<u32> =
    std::cell::LazyCell::new(|| unsafe { (65533 as u32) });
pub unsafe fn test_encode_40(out_it: *mut UnsafeOstreamIterator<libc::c_char>, mut value: u16) {
    if ((value as i32) < (128)) {
        unimplemented!();
    } else if ((value as i32) < (2048)) {
        unimplemented!();
        unimplemented!();
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_encode_41(out_it: *mut UnsafeOstreamIterator<libc::c_char>, mut value: u32) {
    if (((value) & (4294901760_u32)) == (0_u32)) {
        (unsafe {
            let _out_it: *mut UnsafeOstreamIterator<libc::c_char> = out_it;
            let _value: u16 = (value as u16);
            test_encode_40(_out_it, _value)
        });
    } else {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_transcode_42(
    mut first: UnsafeRandomAccessIterator<i32>,
    mut last: UnsafeRandomAccessIterator<i32>,
    mut out_it: UnsafeOstreamIterator<libc::c_char>,
) -> UnsafeOstreamIterator<libc::c_char> {
    'loop_: while first != last {
        let mut value: u32 = ((*first.iter_post_inc().iter_deref()) as u32);
        if (unsafe { test_is_code_point_37(value) }) {
            (unsafe { test_encode_41(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_40(
                    &mut out_it,
                    ((*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_39))
                        as u16),
                )
            });
        }
    }
    return out_it.clone();
}
pub unsafe fn test_transcode_43(
    mut first: *const i32,
    mut last: *const i32,
    mut out_it: UnsafeOstreamIterator<libc::c_char>,
) -> UnsafeOstreamIterator<libc::c_char> {
    'loop_: while ((first) != (last)) {
        let mut value: u32 = ((*first.postfix_inc()) as u32);
        if (unsafe { test_is_code_point_37(value) }) {
            (unsafe { test_encode_41(&mut out_it, value) });
        } else {
            (unsafe {
                test_encode_40(
                    &mut out_it,
                    ((*std::cell::LazyCell::force_mut(&mut *&raw mut test_replacement_character_39))
                        as u16),
                )
            });
        }
    }
    return out_it.clone();
}
pub unsafe fn operator_shl_44(os: *mut ::std::fs::File, loc: *const ()) -> *mut ::std::fs::File {
    return &mut (*{
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*{
                        let __r: &mut ::std::fs::File = &mut (*os);
                        let __s: *mut ::std::fs::File = __r;
                        let _ = (*__s)
                            .write_all(::std::ffi::CStr::from_ptr(unimplemented!()).to_bytes());
                        __s
                    });
                    let __s: *mut ::std::fs::File = __r;
                    let _ = (*__s).write_all(&[(':' as libc::c_char) as u8]);
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                libcc2rs::write_int(&mut *__s, unimplemented!());
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            let _ = (*__s).write_all(&[(':' as libc::c_char) as u8]);
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(&mut *__s, unimplemented!());
        __s
    });
}
pub unsafe fn test_concat_45(
    stream: *mut ::std::fs::File,
    value: *mut libc::c_char,
) -> *mut ::std::fs::File {
    return &mut (*{
        let __r: &mut ::std::fs::File = &mut (*stream);
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(&[(*value) as u8]);
        __s
    });
}
pub unsafe fn test_concat_46(
    stream: *mut ::std::fs::File,
    value: *mut Vec<libc::c_char>,
) -> *mut ::std::fs::File {
    return &mut (*{
        let __r: &mut ::std::fs::File = &mut (*stream);
        let __s: *mut ::std::fs::File = __r;
        let mut __bytes = Vec::new();
        for &__c in &(*value)[..(*value).len().saturating_sub(1)] {
            __c.push_stream_bytes(&mut __bytes);
        }
        let _ = ::std::io::Write::write_all(&mut *__s, &__bytes);
        __s
    });
}
pub unsafe fn test_concat_47(
    stream: *mut ::std::fs::File,
    value: *const [libc::c_char; 18],
) -> *mut ::std::fs::File {
    return &mut (*{
        let __r: &mut ::std::fs::File = &mut (*stream);
        let __s: *mut ::std::fs::File = __r;
        let _ = (*__s).write_all(::std::ffi::CStr::from_ptr((*value).as_ptr()).to_bytes());
        __s
    });
}
pub unsafe fn test_concat_48(stream: *mut ::std::fs::File, value: *mut ()) -> *mut ::std::fs::File {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_49(stream: *mut ::std::fs::File, value: *mut ()) -> *mut ::std::fs::File {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_50(
    stream: *mut ::std::fs::File,
    value: *mut Vec<i32>,
) -> *mut ::std::fs::File {
    let mut b: UnsafeRandomAccessIterator<i32> = unimplemented!();
    let mut e: UnsafeRandomAccessIterator<i32> = unimplemented!();
    if b != e {
        {}
        (unsafe {
            test_transcode_42(b.clone(), e.clone(), {
                let __r: &mut ::std::fs::File = &mut (*stream);
                UnsafeOstreamIterator::new(__r, ::std::ptr::null())
            })
        });
    }
    return stream;
}
pub unsafe fn test_concat_51(stream: *mut ::std::fs::File, value: *mut ()) -> *mut ::std::fs::File {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ((b) != (e)) {
        {}
        (unsafe {
            test_transcode_43(
                (b as *const ()).cast::<i32>(),
                (e as *const ()).cast::<i32>(),
                {
                    let __r: &mut ::std::fs::File = &mut (*stream);
                    UnsafeOstreamIterator::new(__r, ::std::ptr::null())
                },
            )
        });
    }
    return stream;
}
pub unsafe fn test_concat_52(stream: *mut ::std::fs::File, value: *mut ()) -> *mut ::std::fs::File {
    let mut b: () = unimplemented!();
    let mut e: () = unimplemented!();
    if ((b) != (e)) {
        {}
        (unsafe {
            test_transcode_43(
                (b as *const ()).cast::<i32>(),
                (e as *const ()).cast::<i32>(),
                {
                    let __r: &mut ::std::fs::File = &mut (*stream);
                    UnsafeOstreamIterator::new(__r, ::std::ptr::null())
                },
            )
        });
    }
    return stream;
}
pub unsafe fn test_concat_message_53(
    args_0: *const [libc::c_char; 18],
    args_1: *mut (),
    args_2: *const [libc::c_char; 18],
    args_3: *mut (),
    args_4: *const [libc::c_char; 18],
    args_5: *mut Vec<libc::c_char>,
    args_6: *mut libc::c_char,
) -> Vec<libc::c_char> {
    {
        let mut sstr: () = unimplemented!();
        {
            (unsafe {
                let _stream: *mut ::std::fs::File = &mut sstr;
                let _value: *const [libc::c_char; 18] = args_0;
                test_concat_47(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut ::std::fs::File = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_49(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut ::std::fs::File = &mut sstr;
                        let _value: *const [libc::c_char; 18] = args_2;
                        test_concat_47(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut ::std::fs::File = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_48(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut ::std::fs::File = &mut sstr;
                                let _value: *const [libc::c_char; 18] = args_4;
                                test_concat_47(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut ::std::fs::File = &mut sstr;
                                    let _value: *mut Vec<libc::c_char> = args_5;
                                    test_concat_46(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut ::std::fs::File = &mut sstr;
                                    let _value: *mut libc::c_char = args_6;
                                    test_concat_45(_stream, _value)
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
pub unsafe fn test_concat_message_54(
    args_0: *const [libc::c_char; 18],
    args_1: *mut (),
    args_2: *const [libc::c_char; 18],
    args_3: *mut (),
    args_4: *const [libc::c_char; 18],
    args_5: *mut Vec<i32>,
    args_6: *mut libc::c_char,
) -> Vec<libc::c_char> {
    {
        let mut sstr: () = unimplemented!();
        {
            (unsafe {
                let _stream: *mut ::std::fs::File = &mut sstr;
                let _value: *const [libc::c_char; 18] = args_0;
                test_concat_47(_stream, _value)
            });
            {
                (unsafe {
                    let _stream: *mut ::std::fs::File = &mut sstr;
                    let _value: *mut () = args_1;
                    test_concat_52(_stream, _value)
                });
                {
                    (unsafe {
                        let _stream: *mut ::std::fs::File = &mut sstr;
                        let _value: *const [libc::c_char; 18] = args_2;
                        test_concat_47(_stream, _value)
                    });
                    {
                        (unsafe {
                            let _stream: *mut ::std::fs::File = &mut sstr;
                            let _value: *mut () = args_3;
                            test_concat_51(_stream, _value)
                        });
                        {
                            (unsafe {
                                let _stream: *mut ::std::fs::File = &mut sstr;
                                let _value: *const [libc::c_char; 18] = args_4;
                                test_concat_47(_stream, _value)
                            });
                            {
                                (unsafe {
                                    let _stream: *mut ::std::fs::File = &mut sstr;
                                    let _value: *mut Vec<i32> = args_5;
                                    test_concat_50(_stream, _value)
                                });
                                (unsafe {
                                    let _stream: *mut ::std::fs::File = &mut sstr;
                                    let _value: *mut libc::c_char = args_6;
                                    test_concat_45(_stream, _value)
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
pub static mut test_format_55: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (), mut fmt: (), args: *const [libc::c_char; 2]| {
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<libc::c_char> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_53(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_19(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *const [libc::c_char; 3]| {
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<libc::c_char> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_53(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_20(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *const [libc::c_char; 4]| {
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<libc::c_char> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_53(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_21(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *const [libc::c_char; 5]| {
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<libc::c_char> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_53(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_22(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *const [libc::c_char; 9]| {
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<libc::c_char> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_53(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_23(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *const [i32; 2]| {
                {
                    let mut out: Vec<i32> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<i32> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_54(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_24(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<i32> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (),
             mut fmt: (),
             args_2: *mut libc::c_char,
             args_3: *mut libc::c_char| {
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<libc::c_char> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_53(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_25(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *mut libc::c_char| {
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<libc::c_char> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_53(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_26(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<libc::c_char> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_53(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_27(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<libc::c_char> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args_2: *mut i32, args_3: *mut i32| {
                {
                    let mut out: Vec<i32> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<i32> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_54(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_28(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<i32> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *mut i32| {
                {
                    let mut out: Vec<i32> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<i32> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_54(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_29(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<i32> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut out: Vec<i32> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<i32> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_54(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_30(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<i32> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *const [i32; 3]| {
                {
                    let mut out: Vec<i32> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<i32> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_54(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_31(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<i32> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *const [i32; 4]| {
                {
                    let mut out: Vec<i32> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<i32> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_54(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_32(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<i32> = unimplemented!();
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *const [i32; 5]| {
                {
                    let mut out: Vec<i32> = unimplemented!();
                    (unsafe {
                        let _condition: bool = unimplemented!();
                        let mut _arg: FnPtr<fn()> = lambda_unsafe!(
                            {
                                let fmt: *mut () = &mut fmt;
                                let expected: *mut () = &mut expected;
                                let out: *mut Vec<i32> = &mut out;
                            },
                            || {
                                (unsafe {
                                    let mut _args: *const libc::c_char = ((unsafe {
                                        let mut _args_1: () = unimplemented!();
                                        let mut _args_6: libc::c_char = ('\n' as libc::c_char);
                                        test_concat_message_54(
                                            &std::mem::transmute(*b"\nFormat string   \0"),
                                            &mut _args_1,
                                            &std::mem::transmute(*b"\nExpected output \0"),
                                            expected,
                                            &std::mem::transmute(*b"\nActual output   \0"),
                                            out,
                                            &mut _args_6,
                                        )
                                    })
                                    .as_ptr()
                                        as *const libc::c_char);
                                    test_eprintf_2(c"%s".as_ptr(), &mut _args)
                                });
                            }
                        );
                        test_require_33(
                            _condition,
                            c"out == expected".as_ptr(),
                            c"escaped_output.unicode.pass.cpp".as_ptr(),
                            50,
                            &mut _arg,
                        )
                    });
                }
                {
                    let mut out: Vec<i32> = unimplemented!();
                    assert!(unimplemented!());
                }
            }
        )
    });
pub static mut test_format_to_56: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut out: Vec<libc::c_char> =
                        vec![(' ' as libc::c_char); (unimplemented!() as usize) as usize]
                            .iter()
                            .cloned()
                            .chain(std::iter::once((0 as libc::c_char)))
                            .collect();
                    let mut it: UnsafeRandomAccessIterator<libc::c_char> = unimplemented!();
                    assert!(
                        it == UnsafeRandomAccessIterator::Vec(out.as_mut_ptr().add(out.len() - 1))
                    );
                    assert!(unimplemented!());
                }
                {
                    let mut out: Vec<libc::c_char> =
                        vec![(' ' as libc::c_char); (unimplemented!() as usize) as usize]
                            .iter()
                            .cloned()
                            .chain(std::iter::once((0 as libc::c_char)))
                            .collect();
                    let mut it: UnsafeRandomAccessIterator<libc::c_char> = unimplemented!();
                    assert!(
                        it == UnsafeRandomAccessIterator::Vec(out.as_mut_ptr().add(out.len() - 1))
                    );
                    assert!(unimplemented!());
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                    assert!(unimplemented!());
                }
                {
                    let mut out: Vec<libc::c_char> = Vec::new();
                    unimplemented!();
                    assert!(unimplemented!());
                }
                {
                    assert!(
                        ((unimplemented!()) < (4096_()))
                            && (!(c"Update the size of the buffer.".as_ptr()).is_null())
                    );
                    let mut out: [libc::c_char; 4096] = [(0 as libc::c_char); 4096];
                    let mut it: *mut libc::c_char = unimplemented!();
                    assert!(((unimplemented!()) == ((unimplemented!() as i32) as isize)));
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut out: Vec<i32> =
                        vec![((' ' as libc::c_char) as i32); (unimplemented!() as usize) as usize]
                            .iter()
                            .cloned()
                            .chain(std::iter::once(0_i32))
                            .collect();
                    let mut it: UnsafeRandomAccessIterator<i32> = unimplemented!();
                    assert!(
                        it == UnsafeRandomAccessIterator::Vec(out.as_mut_ptr().add(out.len() - 1))
                    );
                    assert!(unimplemented!());
                }
                {
                    let mut out: Vec<i32> =
                        vec![((' ' as libc::c_char) as i32); (unimplemented!() as usize) as usize]
                            .iter()
                            .cloned()
                            .chain(std::iter::once(0_i32))
                            .collect();
                    let mut it: UnsafeRandomAccessIterator<i32> = unimplemented!();
                    assert!(
                        it == UnsafeRandomAccessIterator::Vec(out.as_mut_ptr().add(out.len() - 1))
                    );
                    assert!(unimplemented!());
                }
                {
                    let mut out: () = unimplemented!();
                    unimplemented!();
                    assert!(unimplemented!());
                }
                {
                    let mut out: Vec<i32> = Vec::new();
                    unimplemented!();
                    assert!(unimplemented!());
                }
                {
                    assert!(
                        ((unimplemented!()) < (4096_()))
                            && (!(c"Update the size of the buffer.".as_ptr()).is_null())
                    );
                    let mut out: [i32; 4096] = [0_i32; 4096];
                    let mut it: *mut i32 = unimplemented!();
                    assert!(((unimplemented!()) == ((unimplemented!() as i32) as isize)));
                    assert!(unimplemented!());
                }
            }
        )
    });
pub static mut test_formatted_size_57: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut size: usize = unimplemented!();
                    assert!(((size) == (unimplemented!() as usize)));
                }
                {
                    let mut size: usize = unimplemented!();
                    assert!(((size) == (unimplemented!() as usize)));
                }
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut size: usize = unimplemented!();
                    assert!(((size) == (unimplemented!() as usize)));
                }
                {
                    let mut size: usize = unimplemented!();
                    assert!(((size) == (unimplemented!() as usize)));
                }
            }
        )
    });
pub static mut test_format_to_n_58: std::cell::LazyCell<FnPtr<Generic>> =
    std::cell::LazyCell::new(|| unsafe {
        lambda_unsafe!(
            Generic,
            {},
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut n: usize = (unimplemented!() as usize);
                    let mut out: Vec<libc::c_char> = vec![(' ' as libc::c_char); (n) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once((0 as libc::c_char)))
                        .collect();
                    let mut result: () = unimplemented!();
                    assert!(((result.size as isize) == (unimplemented!() as isize)));
                    assert!(
                        result.out
                            == UnsafeRandomAccessIterator::Vec(out.as_mut_ptr().add(out.len() - 1))
                    );
                    assert!(unimplemented!());
                }
                {
                    let mut n: usize = (unimplemented!() as usize);
                    let mut out: Vec<libc::c_char> = vec![(' ' as libc::c_char); (n) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once((0 as libc::c_char)))
                        .collect();
                    let mut result: () = unimplemented!();
                    assert!(((result.size as isize) == (unimplemented!() as isize)));
                    assert!(
                        result.out
                            == UnsafeRandomAccessIterator::Vec(out.as_mut_ptr().add(out.len() - 1))
                    );
                    assert!(unimplemented!());
                }
                {
                    let mut n: isize = 0_isize;
                    let mut out: Vec<libc::c_char> = vec![libc::c_char::default()];
                    let mut result: () = unimplemented!();
                    assert!(((result.size as isize) == (unimplemented!() as isize)));
                    assert!(
                        result.out
                            == UnsafeRandomAccessIterator::Vec(out.as_mut_ptr().add(out.len() - 1))
                    );
                    assert!(Vec::len(&out) <= 1);
                }
                {
                    let mut n: isize = (((unimplemented!()).wrapping_div(2_())) as isize);
                    let mut out: Vec<libc::c_char> =
                        vec![(' ' as libc::c_char); (n as usize) as usize]
                            .iter()
                            .cloned()
                            .chain(std::iter::once((0 as libc::c_char)))
                            .collect();
                    let mut result: () = unimplemented!();
                    assert!(((result.size as isize) == (unimplemented!() as isize)));
                    assert!(
                        result.out
                            == UnsafeRandomAccessIterator::Vec(out.as_mut_ptr().add(out.len() - 1))
                    );
                    assert!(unimplemented!());
                }
            },
            |mut expected: (), mut fmt: (), args: *mut ()| {
                {
                    let mut n: usize = (unimplemented!() as usize);
                    let mut out: Vec<i32> = vec![((' ' as libc::c_char) as i32); (n) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0_i32))
                        .collect();
                    let mut result: () = unimplemented!();
                    assert!(((result.size as isize) == (unimplemented!() as isize)));
                    assert!(
                        result.out
                            == UnsafeRandomAccessIterator::Vec(out.as_mut_ptr().add(out.len() - 1))
                    );
                    assert!(unimplemented!());
                }
                {
                    let mut n: usize = (unimplemented!() as usize);
                    let mut out: Vec<i32> = vec![((' ' as libc::c_char) as i32); (n) as usize]
                        .iter()
                        .cloned()
                        .chain(std::iter::once(0_i32))
                        .collect();
                    let mut result: () = unimplemented!();
                    assert!(((result.size as isize) == (unimplemented!() as isize)));
                    assert!(
                        result.out
                            == UnsafeRandomAccessIterator::Vec(out.as_mut_ptr().add(out.len() - 1))
                    );
                    assert!(unimplemented!());
                }
                {
                    let mut n: isize = 0_isize;
                    let mut out: Vec<i32> = vec![i32::default()];
                    let mut result: () = unimplemented!();
                    assert!(((result.size as isize) == (unimplemented!() as isize)));
                    assert!(
                        result.out
                            == UnsafeRandomAccessIterator::Vec(out.as_mut_ptr().add(out.len() - 1))
                    );
                    assert!(Vec::len(&out) <= 1);
                }
                {
                    let mut n: isize = (((unimplemented!()).wrapping_div(2_())) as isize);
                    let mut out: Vec<i32> =
                        vec![((' ' as libc::c_char) as i32); (n as usize) as usize]
                            .iter()
                            .cloned()
                            .chain(std::iter::once(0_i32))
                            .collect();
                    let mut result: () = unimplemented!();
                    assert!(((result.size as isize) == (unimplemented!() as isize)));
                    assert!(
                        result.out
                            == UnsafeRandomAccessIterator::Vec(out.as_mut_ptr().add(out.len() - 1))
                    );
                    assert!(unimplemented!());
                }
            }
        )
    });
pub unsafe fn test_char_59() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_2: libc::c_char = ('\'' as libc::c_char);
        let mut _args_3: libc::c_char = ('\"' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char, *mut libc::c_char)>(6)
            .call(_expected, _fmt, &mut _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: libc::c_char = ('\t' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char)>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: libc::c_char = ('\n' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char)>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: libc::c_char = ('\r' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char)>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: libc::c_char = ('\\' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char)>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: libc::c_char = ('\'' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char)>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: libc::c_char = ('\"' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char)>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: libc::c_char = (' ' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char)>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: libc::c_char = ('a' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char)>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: libc::c_char = ('b' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char)>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: libc::c_char = ('c' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char)>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: libc::c_char = ('\0' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char)>(7)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: libc::c_char = ('\x1f' as libc::c_char);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut libc::c_char)>(7)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            let mut _args: libc::c_char = (b'\x80' as libc::c_char);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut libc::c_char)>(7)
                .call(_expected, _fmt, &mut _args)
        });
    }
    {}
    {}
}
pub unsafe fn test_char_60() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args_2: i32 = (('\'' as libc::c_char) as i32);
        let mut _args_3: i32 = (('\"' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32, *mut i32)>(9)
            .call(_expected, _fmt, &mut _args_2, &mut _args_3)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: i32 = (('\t' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32)>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: i32 = (('\n' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32)>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: i32 = (('\r' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32)>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: i32 = (('\\' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32)>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: i32 = (('\'' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32)>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: i32 = (('\"' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32)>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: i32 = ((' ' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32)>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: i32 = (('a' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32)>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: i32 = (('b' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32)>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: i32 = (('c' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32)>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: i32 = (('\0' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32)>(10)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: i32 = (('\x1f' as libc::c_char) as i32);
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut i32)>(10)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    {
        (unsafe {
            let mut _args: i32 = (160 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (12288 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (8232 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (8233 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (173 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (1536 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (65279 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (55296 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (57343 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (57344 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (63743 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (888 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (6004 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (65535 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (768 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (65056 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
    }
    {
        const _: () = assert!(
            ((::std::mem::size_of::<i32>()) == (4_usize)),
            "sizeof(CharT) == 4"
        );;
        let x: i32 = 131071_i32;
        let y: u32 = 131071_u32;
        const _: () = assert!(true, "x == y");;
        (unsafe {
            let mut _args: i32 = (69821 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (917631 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (983040 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (1048573 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (1048576 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (1114109 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (65548 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (1048575 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (1114110 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (66045 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (917760 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (1114112 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
        (unsafe {
            let mut _args: i32 = (4294967295 as i32);
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *mut i32)>(10)
                .call(unimplemented!(), unimplemented!(), &mut _args)
        });
    }
}
pub unsafe fn test_string_61() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 2])>(0)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xc3\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 3])>(1)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xc3(\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 3])>(1)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xc0\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 3])>(1)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xc1\xbf\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 3])>(1)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xc2\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xe0\x80\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xe0\x81\xbf\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xe0\x82\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xe0\x9f\xbf\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xe0\xa0\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xed\x9f\xbf\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xed\xa0\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xed\xaf\xbf\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xed\xbf\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xed\xbf\xbf\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 4])>(2)
                .call(_expected, _fmt, &std::mem::transmute(*b"\xee\x80\x80\0"))
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x80\x80\x80\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x80\x81\xbf\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x80\x82\x80\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x80\x9f\xbf\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x80\xa0\x80\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x8f\xbf\xbf\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf0\x90\x80\x80\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf4\x8f\xbf\xbf\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf4\x90\x80\x80\0"),
                )
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 5])>(3)
                .call(
                    _expected,
                    _fmt,
                    &std::mem::transmute(*b"\xf4\xbf\xbf\xbf\0"),
                )
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [libc::c_char; 2])>(0)
                .call(_expected, _fmt, &std::mem::transmute(*b"\x80\0"))
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    {}
}
pub unsafe fn test_string_62() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 2])>(5)
                .call(_expected, _fmt, &[195 as i32, 0 as i32])
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _fmt: () = unimplemented!();
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 3])>(12)
                .call(_expected, _fmt, &[195 as i32, 40 as i32, 0 as i32])
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    {
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 3])>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[160 as i32, 12288 as i32, 0 as i32],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 2])>(5)
                .call(unimplemented!(), unimplemented!(), &[8232 as i32, 0 as i32])
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 2])>(5)
                .call(unimplemented!(), unimplemented!(), &[8233 as i32, 0 as i32])
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 4])>(13)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[173 as i32, 1536 as i32, 65279 as i32, 0 as i32],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 2])>(5)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[55296 as i32, 0 as i32],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 3])>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[57344 as i32, 63743 as i32, 0 as i32],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 4])>(13)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[888 as i32, 6004 as i32, 65535 as i32, 0 as i32],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 3])>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[768 as i32, 65056 as i32, 0 as i32],
                )
        });
    }
    {
        const _: () = assert!(
            ((::std::mem::size_of::<i32>()) == (4_usize)),
            "sizeof(CharT) == 4"
        );;
        let x: i32 = 131071_i32;
        let y: u32 = 131071_u32;
        const _: () = assert!(true, "x == y");;
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 3])>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[69821 as i32, 917631 as i32, 0 as i32],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 5])>(14)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[
                        983040 as i32,
                        1048573 as i32,
                        1048576 as i32,
                        1114109 as i32,
                        0 as i32,
                    ],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 4])>(13)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[65548 as i32, 1048575 as i32, 1114110 as i32, 0 as i32],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 3])>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[66045 as i32, 917760 as i32, 0 as i32],
                )
        });
        (unsafe {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
                .spec::<fn((), (), *const [i32; 3])>(12)
                .call(
                    unimplemented!(),
                    unimplemented!(),
                    &[1114112 as i32, 4294967295 as i32, 0 as i32],
                )
        });
    }
}
pub unsafe fn test_format_functions_63(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(8)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_64(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_65(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_66(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(0)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_67(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(11)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_68(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_69(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_format_functions_70(mut check: FnPtr<Generic>) {
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _fmt: () = unimplemented!();
        let mut _args: () = unimplemented!();
        check
            .spec::<fn((), (), *mut ())>(1)
            .call(_expected, _fmt, &mut _args)
    });
}
pub unsafe fn test_71() {
    (unsafe { test_char_59() });
    (unsafe { test_string_61() });
    (unsafe {
        test_format_functions_63(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_64(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_to_56)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_65(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_formatted_size_57)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_66(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_to_n_58)).copy_from(),
        )
    });
}
pub unsafe fn test_72() {
    (unsafe { test_char_60() });
    (unsafe { test_string_62() });
    (unsafe {
        test_format_functions_67(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_68(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_to_56)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_69(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_formatted_size_57)).copy_from(),
        )
    });
    (unsafe {
        test_format_functions_70(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_to_n_58)).copy_from(),
        )
    });
}
pub unsafe fn test_ill_formed_utf8_73() {
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 2])>(0)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xdf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 2])>(0)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xef\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 3])>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xef\xbf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 2])>(0)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 3])>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7\xbf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 4])>(2)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7\xbf\xbf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 3])>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xdfa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 3])>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xefa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 4])>(2)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xef\xbfa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 3])>(1)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7a\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 4])>(2)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7\xbfa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 5])>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7\xbf\xbfa\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 9])>(4)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"a\xf1\x80\x80\xe1\x80\xc2b\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 5])>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf4\x8f\xbf\xbf\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 5])>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf4\x90\x80\x80\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 5])>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf5\xb1\xb2\xb3\0"),
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [libc::c_char; 5])>(3)
            .call(
                unimplemented!(),
                unimplemented!(),
                &std::mem::transmute(*b"\xf7\xbf\xbf\xbf\0"),
            )
    });
}
pub unsafe fn test_ill_formed_utf32_74() {
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [i32; 2])>(5)
            .call(
                unimplemented!(),
                unimplemented!(),
                &[1114111 as i32, 0 as i32],
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [i32; 2])>(5)
            .call(
                unimplemented!(),
                unimplemented!(),
                &[1114112 as i32, 0 as i32],
            )
    });
    (unsafe {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut test_format_55))
            .spec::<fn((), (), *const [i32; 2])>(5)
            .call(
                unimplemented!(),
                unimplemented!(),
                &[4294967295 as i32, 0 as i32],
            )
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_71() });
    (unsafe { test_72() });
    (unsafe { test_ill_formed_utf8_73() });
    (unsafe { test_ill_formed_utf32_74() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_39);
    std::cell::LazyCell::force(&*&raw const test_format_55);
    std::cell::LazyCell::force(&*&raw const test_format_to_56);
    std::cell::LazyCell::force(&*&raw const test_formatted_size_57);
    std::cell::LazyCell::force(&*&raw const test_format_to_n_58);
}
