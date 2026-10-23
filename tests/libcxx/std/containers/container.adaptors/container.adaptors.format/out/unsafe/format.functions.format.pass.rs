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
    std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
        { 2_() },
        { (':' as ()) },
        None,
    )
});
static mut result_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    std_basic_string_wchar_t__std_char_traits_wchar_t___std_allocator_wchar_t__::new_1(
        { 1_() },
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
    let mut Name: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
        { c"libcxx.XXXXXX".as_ptr() },
        None,
    );
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
pub unsafe fn test_eprintf_26(
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
pub unsafe fn test_eprintf_27(
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
pub unsafe fn test_eprintf_28(mut fmt: *const (), args: *const *const ()) {
    (unsafe {
        libc::fprintf(
            libcc2rs::stderr_unsafe() as *mut ::libc::FILE,
            fmt as *const libc::c_char,
            (*args),
        )
    });
}
pub unsafe fn test_log_29(
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
        test_eprintf_26(
            c"%s%s %s %d\n%s".as_ptr(),
            &msg,
            &condition,
            &file,
            &line,
            &message,
        )
    });
}
pub unsafe fn test_is_high_surrogate_30(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (56319_()));
}
pub unsafe fn test_is_low_surrogate_31(mut value: ()) -> () {
    return ((value) >= (56320_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_surrogate_32(mut value: ()) -> () {
    return ((value) >= (55296_())) && ((value) <= (57343_()));
}
pub unsafe fn test_is_code_point_33(mut value: ()) -> () {
    return ((value) <= (1114111_()));
}
pub unsafe fn test_is_scalar_value_34(mut value: ()) -> () {
    return (unsafe { test_is_code_point_33(value) })
        && (!(unsafe { test_is_surrogate_32(value) }));
}
pub static mut test_replacement_character_35: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { (65533 as ()) });
pub unsafe fn operator_shl_36(os: *mut (), loc: *const ()) -> *mut () {
    return &mut (*unimplemented!());
}
pub unsafe fn test_concat_message_37(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_38(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_39(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_40(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_41(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_42(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_43(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_44(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_45(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_46(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_47(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_48(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_49(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_50(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_51(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_52(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_53(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_54(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_55(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_56(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_57(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_58(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_59(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_60(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_61(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_62(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_63(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_64(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_65(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_66(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_67(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_68(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_69(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_70(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_71(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_72(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_73(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_74(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_75(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_76(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_77(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_78(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_79(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
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
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
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
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_82(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_83(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_84(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_85(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_86(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_87(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_88(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_89(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_90(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_91(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_92(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_93(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_94(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_95(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_96(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_97(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_98(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_99(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_100(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_101(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_102(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_103(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_104(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_105(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_106(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_107(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_108(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_109(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_110(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_111(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_112(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_113(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_114(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_115(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_116(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_117(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_118(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_119(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_120(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_121(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_122(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_123(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_124(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_125(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_126(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_127(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_128(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
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
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
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
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_131(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_132(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_133(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_134(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_135(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_136(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_137(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_138(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_139(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_140(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_141(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_142(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_143(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_144(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_145(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_146(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_147(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_148(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_149(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_150(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_151(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_152(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_153(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_154(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_155(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_156(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_157(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_158(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_159(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_160(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_161(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_162(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_163(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_164(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_165(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_166(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_167(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_168(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_169(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_170(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_171(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_172(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_173(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_174(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_175(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_176(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_177(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_178(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_179(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_180(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_181(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_182(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_183(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_184(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_185(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_186(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_187(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_188(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub unsafe fn test_concat_message_189(
    args_0: *const [(); 18],
    args_1: *mut (),
    args_2: *const [(); 18],
    args_3: *mut (),
    args_4: *const [(); 18],
    args_5: *mut (),
    args_6: *mut (),
) -> () {
    {
        return std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
            { c"Message discarded since it can\'t be streamed to std::cerr.\n".as_ptr() },
            None,
        );
    }
    panic!("ub: non-void function does not return a value")
}
pub static mut test_190: std::cell::LazyCell<FnPtr<Generic>> = std::cell::LazyCell::new(
    || unsafe {
        lambda_unsafe! ( Generic , { } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_char__std_deque_char__std_allocator_char____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_char__std_deque_char__std_allocator_char____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_191 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_char__std_deque_char__std_allocator_char____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_char__std_deque_char__std_allocator_char____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_192 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_char__std_deque_char__std_allocator_char____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_char__std_deque_char__std_allocator_char____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_193 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_char__std_vector_char__std_allocator_char____std_greater_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_char__std_vector_char__std_allocator_char____std_greater_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_194 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_char__std_vector_char__std_allocator_char____std_greater_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_char__std_vector_char__std_allocator_char____std_greater_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_195 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_char__std_vector_char__std_allocator_char____std_greater_void___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_char__std_vector_char__std_allocator_char____std_greater_void___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_196 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_char__std_deque_char__std_allocator_char____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_char__std_deque_char__std_allocator_char____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_197 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_char__std_deque_char__std_allocator_char____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_char__std_deque_char__std_allocator_char____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_198 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_char__std_deque_char__std_allocator_char____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_char__std_deque_char__std_allocator_char____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_199 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_char__std_string__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_char__std_string__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_200 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_char__std_string__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_char__std_string__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_201 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_char__std_string__ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_char__std_string__ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_202 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_char__std_string__std_greater_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_char__std_string__std_greater_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_203 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_char__std_string__std_greater_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_char__std_string__std_greater_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_204 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_char__std_string__std_greater_void___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_char__std_string__std_greater_void___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_205 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_char__std_string__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_char__std_string__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_206 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_char__std_string__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_char__std_string__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_207 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_char__std_string__ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_char__std_string__ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_208 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_char__std_deque_char__std_allocator_char____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_char__std_deque_char__std_allocator_char____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_209 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_char__std_deque_char__std_allocator_char____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_char__std_deque_char__std_allocator_char____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_210 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_char__std_deque_char__std_allocator_char____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_char__std_deque_char__std_allocator_char____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_211 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_char__std_vector_char__std_allocator_char____std_greater_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_char__std_vector_char__std_allocator_char____std_greater_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_212 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_char__std_vector_char__std_allocator_char____std_greater_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_char__std_vector_char__std_allocator_char____std_greater_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_213 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_char__std_vector_char__std_allocator_char____std_greater_void___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_char__std_vector_char__std_allocator_char____std_greater_void___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_214 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_char__std_deque_char__std_allocator_char____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_char__std_deque_char__std_allocator_char____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_215 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_char__std_deque_char__std_allocator_char____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_char__std_deque_char__std_allocator_char____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_216 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_char__std_deque_char__std_allocator_char____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_char__std_deque_char__std_allocator_char____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_217 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_bool__std_deque_bool__std_allocator_bool____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_bool__std_deque_bool__std_allocator_bool____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_218 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_bool__std_deque_bool__std_allocator_bool____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_bool__std_deque_bool__std_allocator_bool____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_219 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_bool__std_deque_bool__std_allocator_bool____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_bool__std_deque_bool__std_allocator_bool____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_220 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_bool__std_vector_bool__std_allocator_bool____std_less_bool___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_bool__std_vector_bool__std_allocator_bool____std_less_bool___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_221 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_bool__std_vector_bool__std_allocator_bool____std_less_bool___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_bool__std_vector_bool__std_allocator_bool____std_less_bool___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_222 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_bool__std_vector_bool__std_allocator_bool____std_less_bool___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_bool__std_vector_bool__std_allocator_bool____std_less_bool___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_223 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_bool__std_deque_bool__std_allocator_bool____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_bool__std_deque_bool__std_allocator_bool____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_224 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_bool__std_deque_bool__std_allocator_bool____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_bool__std_deque_bool__std_allocator_bool____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_225 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_bool__std_deque_bool__std_allocator_bool____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_bool__std_deque_bool__std_allocator_bool____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_226 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_int__std_deque_int__std_allocator_int____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_int__std_deque_int__std_allocator_int____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_227 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_int__std_deque_int__std_allocator_int____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_int__std_deque_int__std_allocator_int____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_228 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_int__std_deque_int__std_allocator_int____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_int__std_deque_int__std_allocator_int____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_229 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_int__std_vector_int__std_allocator_int____std_greater_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_int__std_vector_int__std_allocator_int____std_greater_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_230 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_int__std_vector_int__std_allocator_int____std_greater_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_int__std_vector_int__std_allocator_int____std_greater_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_231 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_int__std_vector_int__std_allocator_int____std_greater_void___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_int__std_vector_int__std_allocator_int____std_greater_void___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_232 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_int__std_deque_int__std_allocator_int____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_int__std_deque_int__std_allocator_int____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_233 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_int__std_deque_int__std_allocator_int____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_int__std_deque_int__std_allocator_int____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_234 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_int__std_deque_int__std_allocator_int____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_int__std_deque_int__std_allocator_int____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_235 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_236 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_237 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_238 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_239 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_240 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_241 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_242 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref__int__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref__int__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_243 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_244 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_245 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_246 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_247 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_248 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_249 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_250 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_void_ptr__std_vector_void_ptr__std_allocator_void_ptr____std_less_void_ptr___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_void_ptr__std_vector_void_ptr__std_allocator_void_ptr____std_less_void_ptr___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_251 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_void_ptr__std_vector_void_ptr__std_allocator_void_ptr____std_less_void_ptr___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_void_ptr__std_vector_void_ptr__std_allocator_void_ptr____std_less_void_ptr___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_252 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_void_ptr__std_vector_void_ptr__std_allocator_void_ptr____std_less_void_ptr___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_void_ptr__std_vector_void_ptr__std_allocator_void_ptr____std_less_void_ptr___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_253 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_254 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_255 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_256 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_std_string__std_deque_std_string__std_allocator_std_string____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_std_string__std_deque_std_string__std_allocator_std_string____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_257 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_std_string__std_deque_std_string__std_allocator_std_string____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_std_string__std_deque_std_string__std_allocator_std_string____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_258 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_std_string__std_deque_std_string__std_allocator_std_string____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_std_string__std_deque_std_string__std_allocator_std_string____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_259 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_std_string__std_vector_std_string__std_allocator_std_string____std_greater_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_std_string__std_vector_std_string__std_allocator_std_string____std_greater_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_260 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_std_string__std_vector_std_string__std_allocator_std_string____std_greater_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_std_string__std_vector_std_string__std_allocator_std_string____std_greater_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_261 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_std_string__std_vector_std_string__std_allocator_std_string____std_greater_void___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_std_string__std_vector_std_string__std_allocator_std_string____std_greater_void___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_262 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_std_string__std_deque_std_string__std_allocator_std_string____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_std_string__std_deque_std_string__std_allocator_std_string____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_263 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_std_string__std_deque_std_string__std_allocator_std_string____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_std_string__std_deque_std_string__std_allocator_std_string____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_264 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_std_string__std_deque_std_string__std_allocator_std_string____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_std_string__std_deque_std_string__std_allocator_std_string____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_265 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_status__std_deque_status__std_allocator_status____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_status__std_deque_status__std_allocator_status____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_266 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_queue_status__std_deque_status__std_allocator_status____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_queue_status__std_deque_status__std_allocator_status____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_267 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_status__std_vector_status__std_allocator_status____std_less_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_status__std_vector_status__std_allocator_status____std_less_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_268 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_priority_queue_status__std_vector_status__std_allocator_status____std_less_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_priority_queue_status__std_vector_status__std_allocator_status____std_less_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_269 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_status__std_deque_status__std_allocator_status____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_status__std_deque_status__std_allocator_status____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_270 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_char__std_stack_status__std_deque_status__std_allocator_status____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_char__std_stack_status__std_deque_status__std_allocator_status____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_271 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_wchar_t__std_deque_wchar_t__std_allocator_wchar_t____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_wchar_t__std_deque_wchar_t__std_allocator_wchar_t____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_272 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_wchar_t__std_deque_wchar_t__std_allocator_wchar_t____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_wchar_t__std_deque_wchar_t__std_allocator_wchar_t____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_273 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_wchar_t__std_deque_wchar_t__std_allocator_wchar_t____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_wchar_t__std_deque_wchar_t__std_allocator_wchar_t____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_274 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_wchar_t__std_vector_wchar_t__std_allocator_wchar_t____std_greater_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_wchar_t__std_vector_wchar_t__std_allocator_wchar_t____std_greater_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_275 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_wchar_t__std_vector_wchar_t__std_allocator_wchar_t____std_greater_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_wchar_t__std_vector_wchar_t__std_allocator_wchar_t____std_greater_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_276 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_wchar_t__std_vector_wchar_t__std_allocator_wchar_t____std_greater_void___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_wchar_t__std_vector_wchar_t__std_allocator_wchar_t____std_greater_void___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_277 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_wchar_t__std_deque_wchar_t__std_allocator_wchar_t____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_wchar_t__std_deque_wchar_t__std_allocator_wchar_t____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_278 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_wchar_t__std_deque_wchar_t__std_allocator_wchar_t____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_wchar_t__std_deque_wchar_t__std_allocator_wchar_t____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_279 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_wchar_t__std_deque_wchar_t__std_allocator_wchar_t____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_wchar_t__std_deque_wchar_t__std_allocator_wchar_t____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_280 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_wchar_t__std_wstring__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_wchar_t__std_wstring__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_281 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_wchar_t__std_wstring__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_wchar_t__std_wstring__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_282 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_wchar_t__std_wstring__ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_wchar_t__std_wstring__ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_283 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_wchar_t__std_wstring__std_greater_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_wchar_t__std_wstring__std_greater_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_284 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_wchar_t__std_wstring__std_greater_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_wchar_t__std_wstring__std_greater_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_285 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_wchar_t__std_wstring__std_greater_void___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_wchar_t__std_wstring__std_greater_void___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_286 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_wchar_t__std_wstring__ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_wchar_t__std_wstring__ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_287 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_wchar_t__std_wstring__ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_wchar_t__std_wstring__ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_288 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_wchar_t__std_wstring__ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_wchar_t__std_wstring__ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_289 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_bool__std_deque_bool__std_allocator_bool____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_bool__std_deque_bool__std_allocator_bool____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_290 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_bool__std_deque_bool__std_allocator_bool____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_bool__std_deque_bool__std_allocator_bool____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_291 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_bool__std_deque_bool__std_allocator_bool____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_bool__std_deque_bool__std_allocator_bool____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_292 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_bool__std_vector_bool__std_allocator_bool____std_less_bool___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_bool__std_vector_bool__std_allocator_bool____std_less_bool___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_293 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_bool__std_vector_bool__std_allocator_bool____std_less_bool___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_bool__std_vector_bool__std_allocator_bool____std_less_bool___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_294 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_bool__std_vector_bool__std_allocator_bool____std_less_bool___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_bool__std_vector_bool__std_allocator_bool____std_less_bool___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_295 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_bool__std_deque_bool__std_allocator_bool____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_bool__std_deque_bool__std_allocator_bool____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_296 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_bool__std_deque_bool__std_allocator_bool____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_bool__std_deque_bool__std_allocator_bool____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_297 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_bool__std_deque_bool__std_allocator_bool____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_bool__std_deque_bool__std_allocator_bool____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_298 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_int__std_deque_int__std_allocator_int____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_int__std_deque_int__std_allocator_int____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_299 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_int__std_deque_int__std_allocator_int____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_int__std_deque_int__std_allocator_int____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_300 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_int__std_deque_int__std_allocator_int____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_int__std_deque_int__std_allocator_int____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_301 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_int__std_vector_int__std_allocator_int____std_greater_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_int__std_vector_int__std_allocator_int____std_greater_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_302 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_int__std_vector_int__std_allocator_int____std_greater_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_int__std_vector_int__std_allocator_int____std_greater_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_303 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_int__std_vector_int__std_allocator_int____std_greater_void___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_int__std_vector_int__std_allocator_int____std_greater_void___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_304 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_int__std_deque_int__std_allocator_int____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_int__std_deque_int__std_allocator_int____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_305 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_int__std_deque_int__std_allocator_int____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_int__std_deque_int__std_allocator_int____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_306 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_int__std_deque_int__std_allocator_int____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_int__std_deque_int__std_allocator_int____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_307 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_308 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_309 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_310 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_311 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_312 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_313 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_314 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref__int__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_long_double__std_vector_long_double__std_allocator_long_double____std_greater_void___ref__int__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_315 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_316 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_317 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_318 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , args_5 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_long_double__std_deque_long_double__std_allocator_long_double____ref__int__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_319 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_320 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_321 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_322 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_void_ptr__std_vector_void_ptr__std_allocator_void_ptr____std_less_void_ptr___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_void_ptr__std_vector_void_ptr__std_allocator_void_ptr____std_less_void_ptr___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_323 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_void_ptr__std_vector_void_ptr__std_allocator_void_ptr____std_less_void_ptr___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_void_ptr__std_vector_void_ptr__std_allocator_void_ptr____std_less_void_ptr___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_324 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_void_ptr__std_vector_void_ptr__std_allocator_void_ptr____std_less_void_ptr___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_void_ptr__std_vector_void_ptr__std_allocator_void_ptr____std_less_void_ptr___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_325 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_326 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_327 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_void_ptr__std_deque_void_ptr__std_allocator_void_ptr____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_328 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_std_wstring__std_deque_std_wstring__std_allocator_std_wstring____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_std_wstring__std_deque_std_wstring__std_allocator_std_wstring____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_329 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_std_wstring__std_deque_std_wstring__std_allocator_std_wstring____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_std_wstring__std_deque_std_wstring__std_allocator_std_wstring____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_330 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_std_wstring__std_deque_std_wstring__std_allocator_std_wstring____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_std_wstring__std_deque_std_wstring__std_allocator_std_wstring____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_331 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_std_wstring__std_vector_std_wstring__std_allocator_std_wstring____std_greater_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_std_wstring__std_vector_std_wstring__std_allocator_std_wstring____std_greater_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_332 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_std_wstring__std_vector_std_wstring__std_allocator_std_wstring____std_greater_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_std_wstring__std_vector_std_wstring__std_allocator_std_wstring____std_greater_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_333 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_std_wstring__std_vector_std_wstring__std_allocator_std_wstring____std_greater_void___ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_std_wstring__std_vector_std_wstring__std_allocator_std_wstring____std_greater_void___ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_334 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_std_wstring__std_deque_std_wstring__std_allocator_std_wstring____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_std_wstring__std_deque_std_wstring__std_allocator_std_wstring____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_335 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_std_wstring__std_deque_std_wstring__std_allocator_std_wstring____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_std_wstring__std_deque_std_wstring__std_allocator_std_wstring____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_336 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_std_wstring__std_deque_std_wstring__std_allocator_std_wstring____ref__int__int_ , args_2 : *mut () , args_3 : *mut () , args_4 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_std_wstring__std_deque_std_wstring__std_allocator_std_wstring____ref__int__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_337 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_status__std_deque_status__std_allocator_status____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_status__std_deque_status__std_allocator_status____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_338 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_queue_status__std_deque_status__std_allocator_status____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_queue_status__std_deque_status__std_allocator_status____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_339 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_status__std_vector_status__std_allocator_status____std_less_void___ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_status__std_vector_status__std_allocator_status____std_less_void___ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_340 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_priority_queue_status__std_vector_status__std_allocator_status____std_less_void___ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_priority_queue_status__std_vector_status__std_allocator_status____std_less_void___ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_341 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_status__std_deque_status__std_allocator_status____ref_ , args : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_status__std_deque_status__std_allocator_status____ref_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_342 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } , | mut expected : () , mut fmt : std_basic_format_string_wchar_t__std_stack_status__std_deque_status__std_allocator_status____ref__int_ , args_2 : *mut () , args_3 : *mut () , | { let mut out : () = unimplemented!()  ;
  ;
 ;
 ( unsafe { let _condition: ()  = unimplemented!()  ; let mut _arg : FnPtr<fn()>  = lambda_unsafe! ( { let fmt : *mut std_basic_format_string_wchar_t__std_stack_status__std_deque_status__std_allocator_status____ref__int_ = & mut fmt ;
 let expected : *mut () = & mut expected ;
 let out : *mut () = & mut out ;
 } , | | { ( unsafe { let mut _args : *const ()  = ( unimplemented!()  as *const () ) ; test_eprintf_28 ( c"%s" .as_ptr() , & mut _args , ) } )  ;
 } ) ; test_require_343 ( _condition , c"out == expected" .as_ptr() , c"format.functions.format.pass.cpp" .as_ptr() , 42_() , & mut _arg , ) } )  ;
 } )
    },
);
pub static mut test_exception_344: std::cell::LazyCell<FnPtr<Generic>> =
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
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
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
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut (), _a4: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut (), _a4: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut (), _a4: *mut ()| {},
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
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
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
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut (), _a4: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut (), _a4: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut (), _a3: *mut (), _a4: *mut ()| {},
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
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {},
            |mut _a0: (), mut _a1: (), _a2: *mut ()| {}
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
        format_tests_345(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_190)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_344)).copy_from(),
        )
    });
    (unsafe {
        format_tests_346(
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_190)).copy_from(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut test_exception_344)).copy_from(),
        )
    });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const test_replacement_character_35);
    std::cell::LazyCell::force(&*&raw const test_190);
    std::cell::LazyCell::force(&*&raw const test_exception_344);
}
