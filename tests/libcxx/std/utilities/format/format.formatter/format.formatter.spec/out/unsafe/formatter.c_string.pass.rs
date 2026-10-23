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
pub unsafe fn test_4(mut expected: (), mut fmt: (), mut a: *const (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut arg: *mut () = a;
    let mut format_ctx: () = (unsafe {
        test_format_context_create_0(out.clone(), {
            let mut __tmp_0 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___char_ptr_  = ( unsafe { make_format_args_5 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_1 ( { & mut __tmp_0 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_6(mut expected: (), mut fmt: (), mut a: *const (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut arg: *const () = a;
    let mut format_ctx: () = (unsafe {
        test_format_context_create_0(out.clone(), {
            let mut __tmp_1 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___const_char_ptr_  = ( unsafe { make_format_args_7 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_2 ( { & mut __tmp_1 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_8(mut expected: (), mut fmt: (), mut a: *const (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut arg: *mut () = a;
    let mut format_ctx: () = (unsafe {
        test_format_context_create_2(out.clone(), {
            let mut __tmp_2 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___wchar_t_ptr_  = ( unsafe { make_format_args_9 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_1 ( { & mut __tmp_2 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_10(mut expected: (), mut fmt: (), mut a: *const (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut arg: *const () = a;
    let mut format_ctx: () = (unsafe {
        test_format_context_create_2(out.clone(), {
            let mut __tmp_3 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___const_wchar_t_ptr_  = ( unsafe { make_format_args_11 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_2 ( { & mut __tmp_3 } , )
        })
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_termination_condition_12(mut expected: (), mut f: (), mut arg: *const ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_4(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_4(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_13(mut expected: (), mut f: (), mut arg: *const ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_6(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_6(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_14(mut expected: (), mut f: (), mut arg: *const ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_8(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_8(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_15(mut expected: (), mut f: (), mut arg: *const ()) {
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_10(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_10(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_set_debug_format_16() -> () {
    let mut formatter: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fmt: () = unimplemented!();
    let mut parse_ctx: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_set_debug_format_17() -> () {
    let mut formatter: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fmt: () = unimplemented!();
    let mut parse_ctx: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_set_debug_format_18() -> () {
    let mut formatter: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fmt: () = unimplemented!();
    let mut parse_ctx: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_set_debug_format_19() -> () {
    let mut formatter: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut fmt: () = unimplemented!();
    let mut parse_ctx: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_char_pointer_20() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c" azAZ09,./<>?".as_ptr() },
                    { (::std::mem::size_of::<[(); 14]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_12(_expected, _f, _arg)
    });
    let mut s: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_12(_expected, _f, unimplemented!())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_12(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_12(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_12(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_12(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_12(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_12(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_12(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"universe".as_ptr() },
                    { (::std::mem::size_of::<[(); 9]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_12(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_12(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"universe".as_ptr() },
                    { (::std::mem::size_of::<[(); 9]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_12(_expected, _f, _arg)
    });
    (unsafe { test_set_debug_format_16() });
    const _: () = assert!(true, "test_set_debug_format<ArgumentT, CharT>()");;
}
pub unsafe fn test_char_pointer_21() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c" azAZ09,./<>?".as_ptr() },
                    { (::std::mem::size_of::<[(); 14]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_13(_expected, _f, _arg)
    });
    let mut s: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_13(_expected, _f, unimplemented!())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_13(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_13(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_13(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_13(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_13(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_13(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_13(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"universe".as_ptr() },
                    { (::std::mem::size_of::<[(); 9]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_13(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_13(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_2(
                &MultiStringType::new(
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"universe".as_ptr() },
                    { (::std::mem::size_of::<[(); 9]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_13(_expected, _f, _arg)
    });
    (unsafe { test_set_debug_format_17() });
    const _: () = assert!(true, "test_set_debug_format<ArgumentT, CharT>()");;
}
pub unsafe fn test_char_pointer_22() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c" azAZ09,./<>?".as_ptr() },
                    { (::std::mem::size_of::<[(); 14]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_14(_expected, _f, _arg)
    });
    let mut s: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_14(_expected, _f, unimplemented!())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_14(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_14(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_14(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_14(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_14(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_14(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_14(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"universe".as_ptr() },
                    { (::std::mem::size_of::<[(); 9]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_14(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_14(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"universe".as_ptr() },
                    { (::std::mem::size_of::<[(); 9]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_14(_expected, _f, _arg)
    });
    (unsafe { test_set_debug_format_18() });
    const _: () = assert!(true, "test_set_debug_format<ArgumentT, CharT>()");;
}
pub unsafe fn test_char_pointer_23() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            32 as (), 97 as (), 122 as (), 65 as (), 90 as (), 48 as (), 57 as (),
                            44 as (), 46 as (), 47 as (), 60 as (), 62 as (), 63 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 14]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c" azAZ09,./<>?".as_ptr() },
                    { (::std::mem::size_of::<[(); 14]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_15(_expected, _f, _arg)
    });
    let mut s: () = unimplemented!();
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_15(_expected, _f, unimplemented!())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_15(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_15(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_15(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_15(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_15(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_15(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_15(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"universe".as_ptr() },
                    { (::std::mem::size_of::<[(); 9]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_15(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            119 as (), 111 as (), 114 as (), 108 as (), 100 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 6]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"world".as_ptr() },
                    { (::std::mem::size_of::<[(); 6]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_15(_expected, _f, _arg)
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        let _arg: *const () = (unsafe {
            MultiStringType::as_ptr_3(
                &MultiStringType::new(
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    {
                        [
                            117 as (), 110 as (), 105 as (), 118 as (), 101 as (), 114 as (),
                            115 as (), 101 as (), 0 as (),
                        ]
                        .as_ptr()
                    },
                    {
                        ((::std::mem::size_of::<[(); 9]>() as ())
                            .wrapping_div((::std::mem::size_of::<()>() as ()))
                            as ())
                            .wrapping_sub(1_())
                    },
                    { c"universe".as_ptr() },
                    { (::std::mem::size_of::<[(); 9]>() as ()).wrapping_sub(1_()) },
                ),
                (std::ptr::null() as *const ()).cast::<()>(),
            )
        });
        test_termination_condition_15(_expected, _f, _arg)
    });
    (unsafe { test_set_debug_format_19() });
    const _: () = assert!(true, "test_set_debug_format<ArgumentT, CharT>()");;
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
    (unsafe { test_char_pointer_20() });
    (unsafe { test_char_pointer_21() });
    (unsafe { test_char_pointer_22() });
    (unsafe { test_char_pointer_23() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
