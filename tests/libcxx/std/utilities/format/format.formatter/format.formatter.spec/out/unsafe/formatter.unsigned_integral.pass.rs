extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_format_context_create_0(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_1(out_it, args.clone(), None) });
}
pub unsafe fn test_format_context_create_2(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_3(out_it, args.clone(), None) });
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
pub unsafe fn test_4(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_0(out.clone(), {
            let mut __tmp_0 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___unsigned_char_  = ( unsafe { make_format_args_5 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_1 ( { & mut __tmp_0 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_6(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_0(out.clone(), {
            let mut __tmp_1 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___unsigned_short_  = ( unsafe { make_format_args_7 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_2 ( { & mut __tmp_1 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_8(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_0(out.clone(), {
            let mut __tmp_2 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___unsigned_int_  = ( unsafe { make_format_args_9 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_3 ( { & mut __tmp_2 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_10(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_0(out.clone(), {
            let mut __tmp_3 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___unsigned_long_  = ( unsafe { make_format_args_11 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_4 ( { & mut __tmp_3 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_12(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_0(out.clone(), {
            let mut __tmp_4 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___unsigned_long_long_  = ( unsafe { make_format_args_13 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_5 ( { & mut __tmp_4 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_14(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_0(out.clone(), {
            let mut __tmp_5 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___unsigned___int128_  = ( unsafe { make_format_args_15 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_6 ( { & mut __tmp_5 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_16(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_2(out.clone(), {
            let mut __tmp_6 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___unsigned_char_  = ( unsafe { make_format_args_17 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_1 ( { & mut __tmp_6 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_18(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_2(out.clone(), {
            let mut __tmp_7 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___unsigned_short_  = ( unsafe { make_format_args_19 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_2 ( { & mut __tmp_7 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_20(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_2(out.clone(), {
            let mut __tmp_8 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___unsigned_int_  = ( unsafe { make_format_args_21 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_3 ( { & mut __tmp_8 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_22(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_2(out.clone(), {
            let mut __tmp_9 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___unsigned_long_  = ( unsafe { make_format_args_23 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_4 ( { & mut __tmp_9 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_24(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_2(out.clone(), {
            let mut __tmp_10 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___unsigned_long_long_  = ( unsafe { make_format_args_25 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_5 ( { & mut __tmp_10 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_26(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_2(out.clone(), {
            let mut __tmp_11 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___unsigned___int128_  = ( unsafe { make_format_args_27 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_6 ( { & mut __tmp_11 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_termination_condition_28(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_4(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_4(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_29(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_6(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_6(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_30(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_8(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_8(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_31(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_10(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_10(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_32(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_12(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_12(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_33(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_14(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_14(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_34(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_16(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_16(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_35(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_18(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_18(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_36(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_20(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_20(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_37(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_22(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_22(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_38(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_24(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_24(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_39(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_26(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_26(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_unsigned_integral_type_40() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_28(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_28(_expected, _f, 255_())
    });
    {}
    {}
    {}
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_28(_expected, _f, 255_())
    });
}
pub unsafe fn test_unsigned_integral_type_41() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_29(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_29(_expected, _f, 255_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_29(_expected, _f, 65535_())
        });
    }
    {}
    {}
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_29(_expected, _f, 255_())
    });
}
pub unsafe fn test_unsigned_integral_type_42() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_30(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_30(_expected, _f, 255_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_30(_expected, _f, 65535_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_30(_expected, _f, 4294967295_())
        });
    }
    {}
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_30(_expected, _f, 255_())
    });
}
pub unsafe fn test_unsigned_integral_type_43() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_31(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_31(_expected, _f, 255_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_31(_expected, _f, 65535_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_31(_expected, _f, 4294967295_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_31(_expected, _f, 8446744073709551615_())
        });
    }
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_31(_expected, _f, 255_())
    });
}
pub unsafe fn test_unsigned_integral_type_44() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_32(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_32(_expected, _f, 255_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_32(_expected, _f, 65535_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_32(_expected, _f, 4294967295_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_32(_expected, _f, 8446744073709551615_())
        });
    }
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_32(_expected, _f, 255_())
    });
}
pub unsafe fn test_unsigned_integral_type_45() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_33(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_33(_expected, _f, 255_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_33(_expected, _f, 65535_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_33(_expected, _f, 4294967295_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_33(_expected, _f, 8446744073709551615_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_33(_expected, _f, unimplemented!())
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_33(_expected, _f, 255_())
    });
}
pub unsafe fn test_unsigned_integral_type_46() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_34(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_34(_expected, _f, 255_())
    });
    {}
    {}
    {}
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_34(_expected, _f, 255_())
    });
}
pub unsafe fn test_unsigned_integral_type_47() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_35(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_35(_expected, _f, 255_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_35(_expected, _f, 65535_())
        });
    }
    {}
    {}
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_35(_expected, _f, 255_())
    });
}
pub unsafe fn test_unsigned_integral_type_48() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_36(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_36(_expected, _f, 255_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_36(_expected, _f, 65535_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_36(_expected, _f, 4294967295_())
        });
    }
    {}
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_36(_expected, _f, 255_())
    });
}
pub unsafe fn test_unsigned_integral_type_49() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_37(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_37(_expected, _f, 255_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_37(_expected, _f, 65535_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_37(_expected, _f, 4294967295_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_37(_expected, _f, 8446744073709551615_())
        });
    }
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_37(_expected, _f, 255_())
    });
}
pub unsafe fn test_unsigned_integral_type_50() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_38(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_38(_expected, _f, 255_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_38(_expected, _f, 65535_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_38(_expected, _f, 4294967295_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_38(_expected, _f, 8446744073709551615_())
        });
    }
    {}
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_38(_expected, _f, 255_())
    });
}
pub unsafe fn test_unsigned_integral_type_51() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_39(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_39(_expected, _f, 255_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_39(_expected, _f, 65535_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_39(_expected, _f, 4294967295_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_39(_expected, _f, 8446744073709551615_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_39(_expected, _f, unimplemented!())
        });
    }
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_39(_expected, _f, 255_())
    });
}
pub unsafe fn test_all_unsigned_integral_types_52() {
    (unsafe { test_unsigned_integral_type_40() });
    (unsafe { test_unsigned_integral_type_41() });
    (unsafe { test_unsigned_integral_type_42() });
    (unsafe { test_unsigned_integral_type_43() });
    (unsafe { test_unsigned_integral_type_44() });
    (unsafe { test_unsigned_integral_type_45() });
}
pub unsafe fn test_all_unsigned_integral_types_53() {
    (unsafe { test_unsigned_integral_type_46() });
    (unsafe { test_unsigned_integral_type_47() });
    (unsafe { test_unsigned_integral_type_48() });
    (unsafe { test_unsigned_integral_type_49() });
    (unsafe { test_unsigned_integral_type_50() });
    (unsafe { test_unsigned_integral_type_51() });
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
    (unsafe { test_all_unsigned_integral_types_52() });
    (unsafe { test_all_unsigned_integral_types_53() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
