extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_format_context_create_0(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_1((*unimplemented!()), args.clone(), None) });
}
pub unsafe fn test_format_context_create_2(mut out_it: (), mut args: ()) -> () {
    return (unsafe { __format_context_create_3((*unimplemented!()), args.clone(), None) });
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
pub unsafe fn test_4(mut fmt: (), mut arg: (), mut expected: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_0(out.clone(), {
            let mut __tmp_0 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___float_  = ( unsafe { make_format_args_5 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_1 ( { & mut __tmp_0 } , )
        })
    });
    unimplemented!();
    if unimplemented!() {
        let mut buffer: () = unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_6(mut fmt: (), mut arg: (), mut expected: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_0(out.clone(), {
            let mut __tmp_1 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___double_  = ( unsafe { make_format_args_7 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_2 ( { & mut __tmp_1 } , )
        })
    });
    unimplemented!();
    if unimplemented!() {
        let mut buffer: () = unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_8(mut fmt: (), mut arg: (), mut expected: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_0(out.clone(), {
            let mut __tmp_2 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_string___char___long_double_  = ( unsafe { make_format_args_9 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_string___char__ :: new_3 ( { & mut __tmp_2 } , )
        })
    });
    unimplemented!();
    if unimplemented!() {
        let mut buffer: () = unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_10(mut fmt: (), mut arg: (), mut expected: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_2(out.clone(), {
            let mut __tmp_3 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___float_  = ( unsafe { make_format_args_11 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_1 ( { & mut __tmp_3 } , )
        })
    });
    unimplemented!();
    if unimplemented!() {
        let mut buffer: () = unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_12(mut fmt: (), mut arg: (), mut expected: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_2(out.clone(), {
            let mut __tmp_4 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___double_  = ( unsafe { make_format_args_13 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_2 ( { & mut __tmp_4 } , )
        })
    });
    unimplemented!();
    if unimplemented!() {
        let mut buffer: () = unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_14(mut fmt: (), mut arg: (), mut expected: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () = (unsafe {
        test_format_context_create_2(out.clone(), {
            let mut __tmp_5 : std___format_arg_store_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t___long_double_  = ( unsafe { make_format_args_15 ( & mut arg , ) } )  ;
            std_basic_format_args_std_basic_format_context_std_back_insert_iterator_std_wstring___wchar_t__ :: new_3 ( { & mut __tmp_5 } , )
        })
    });
    unimplemented!();
    if unimplemented!() {
        let mut buffer: () = unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_termination_condition_16(mut f: (), mut arg: (), mut expected: Option<()>) {
    let mut expected: () = expected.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_4(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    (unsafe { test_4(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub unsafe fn test_termination_condition_17(mut f: (), mut arg: (), mut expected: Option<()>) {
    let mut expected: () = expected.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_6(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    (unsafe { test_6(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub unsafe fn test_termination_condition_18(mut f: (), mut arg: (), mut expected: Option<()>) {
    let mut expected: () = expected.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_8(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    (unsafe { test_8(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub unsafe fn test_termination_condition_19(mut f: (), mut arg: (), mut expected: Option<()>) {
    let mut expected: () = expected.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_10(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    (unsafe { test_10(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub unsafe fn test_termination_condition_20(mut f: (), mut arg: (), mut expected: Option<()>) {
    let mut expected: () = expected.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_12(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    (unsafe { test_12(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub unsafe fn test_termination_condition_21(mut f: (), mut arg: (), mut expected: Option<()>) {
    let mut expected: () = expected.unwrap_or_else(|| unsafe { unimplemented!() });
    let mut fmt: () = (unsafe { f() });
    unimplemented!();
    (unsafe { test_14(unimplemented!(), arg, unimplemented!(), 1_()) });
    unimplemented!();
    (unsafe { test_14(unimplemented!(), arg, unimplemented!(), 0_()) });
}
pub unsafe fn test_hex_lower_case_precision_22(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_hex_lower_case_precision_24(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_hex_lower_case_precision_26(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_hex_lower_case_precision_27(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_hex_upper_case_precision_28(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_hex_upper_case_precision_29(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_hex_upper_case_precision_30(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_hex_upper_case_precision_31(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_scientific_lower_case_precision_32(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_scientific_lower_case_precision_33(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_scientific_lower_case_precision_34(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_scientific_lower_case_precision_35(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_scientific_upper_case_precision_36(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_scientific_upper_case_precision_37(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_scientific_upper_case_precision_38(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_scientific_upper_case_precision_39(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_fixed_lower_case_precision_40(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_fixed_lower_case_precision_41(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_fixed_lower_case_precision_42(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_fixed_lower_case_precision_43(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_fixed_upper_case_precision_44(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_fixed_upper_case_precision_45(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_fixed_upper_case_precision_46(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_fixed_upper_case_precision_47(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_general_lower_case_precision_48(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_general_lower_case_precision_49(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_general_lower_case_precision_50(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_general_lower_case_precision_51(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_general_upper_case_precision_52(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_16(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_general_upper_case_precision_53(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_17(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_general_upper_case_precision_54(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_23(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_19(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_general_upper_case_precision_55(mut value: ()) {
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
    {
        let mut buffer: () = unimplemented!();
        let mut end_ptr: *mut () = unimplemented!().ptr;
        let mut size: () =
            ((end_ptr as usize - unimplemented!() as usize) / ::std::mem::size_of::<()>()) as ();
        let mut end: () = unimplemented!().offset((size) as isize);
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        let mut unused: () =
            ((unimplemented!() as usize - end as usize) / ::std::mem::size_of::<()>()) as ();
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
        unimplemented!();
        if (unsafe { signbit_25(value) }) {
            (*unimplemented!()) = ('-' as ());
            (*unimplemented!()) = ('0' as ());
        }
        (unsafe { test_termination_condition_20(unimplemented!(), value, Some(unimplemented!())) });
    }
}
pub unsafe fn test_value_56(mut value: ()) {
    (unsafe { test_hex_lower_case_precision_22(value) });
    (unsafe { test_hex_upper_case_precision_28(value) });
    (unsafe { test_scientific_lower_case_precision_32(value) });
    (unsafe { test_scientific_upper_case_precision_36(value) });
    (unsafe { test_fixed_lower_case_precision_40(value) });
    (unsafe { test_fixed_upper_case_precision_44(value) });
    (unsafe { test_general_lower_case_precision_48(value) });
    (unsafe { test_general_upper_case_precision_52(value) });
}
pub unsafe fn test_value_57(mut value: ()) {
    (unsafe { test_hex_lower_case_precision_24(value) });
    (unsafe { test_hex_upper_case_precision_29(value) });
    (unsafe { test_scientific_lower_case_precision_33(value) });
    (unsafe { test_scientific_upper_case_precision_37(value) });
    (unsafe { test_fixed_lower_case_precision_41(value) });
    (unsafe { test_fixed_upper_case_precision_45(value) });
    (unsafe { test_general_lower_case_precision_49(value) });
    (unsafe { test_general_upper_case_precision_53(value) });
}
pub unsafe fn test_value_58(mut value: ()) {
    (unsafe { test_hex_lower_case_precision_26(value) });
    (unsafe { test_hex_upper_case_precision_30(value) });
    (unsafe { test_scientific_lower_case_precision_34(value) });
    (unsafe { test_scientific_upper_case_precision_38(value) });
    (unsafe { test_fixed_lower_case_precision_42(value) });
    (unsafe { test_fixed_upper_case_precision_46(value) });
    (unsafe { test_general_lower_case_precision_50(value) });
    (unsafe { test_general_upper_case_precision_54(value) });
}
pub unsafe fn test_value_59(mut value: ()) {
    (unsafe { test_hex_lower_case_precision_27(value) });
    (unsafe { test_hex_upper_case_precision_31(value) });
    (unsafe { test_scientific_lower_case_precision_35(value) });
    (unsafe { test_scientific_upper_case_precision_39(value) });
    (unsafe { test_fixed_lower_case_precision_43(value) });
    (unsafe { test_fixed_upper_case_precision_47(value) });
    (unsafe { test_general_lower_case_precision_51(value) });
    (unsafe { test_general_upper_case_precision_55(value) });
}
pub unsafe fn test_special_values_60() {
    (unsafe { test_value_56(-unimplemented!()) });
    (unsafe { test_value_56(-1_f64) });
    (unsafe { test_value_56(-unimplemented!()) });
    (unsafe { test_value_56(-unimplemented!()) });
    (unsafe { test_value_56(-0_f64) });
    (unsafe { test_value_56(0_f64) });
    (unsafe { test_value_56(unimplemented!()) });
    (unsafe { test_value_56(1_f64) });
    (unsafe { test_value_56(unimplemented!()) });
    (unsafe { test_value_56(unimplemented!()) });
}
pub unsafe fn test_special_values_61() {
    (unsafe { test_value_57(-unimplemented!()) });
    (unsafe { test_value_57(-1_f64) });
    (unsafe { test_value_57(-unimplemented!()) });
    (unsafe { test_value_57(-unimplemented!()) });
    (unsafe { test_value_57(-0_f64) });
    (unsafe { test_value_57(0_f64) });
    (unsafe { test_value_57(unimplemented!()) });
    (unsafe { test_value_57(1_f64) });
    (unsafe { test_value_57(unimplemented!()) });
    (unsafe { test_value_57(unimplemented!()) });
}
pub unsafe fn test_special_values_62() {
    (unsafe { test_value_58(-unimplemented!()) });
    (unsafe { test_value_58(-1_f64) });
    (unsafe { test_value_58(-unimplemented!()) });
    (unsafe { test_value_58(-unimplemented!()) });
    (unsafe { test_value_58(-0_f64) });
    (unsafe { test_value_58(0_f64) });
    (unsafe { test_value_58(unimplemented!()) });
    (unsafe { test_value_58(1_f64) });
    (unsafe { test_value_58(unimplemented!()) });
    (unsafe { test_value_58(unimplemented!()) });
}
pub unsafe fn test_special_values_63() {
    (unsafe { test_value_59(-unimplemented!()) });
    (unsafe { test_value_59(-1_f64) });
    (unsafe { test_value_59(-unimplemented!()) });
    (unsafe { test_value_59(-unimplemented!()) });
    (unsafe { test_value_59(-0_f64) });
    (unsafe { test_value_59(0_f64) });
    (unsafe { test_value_59(unimplemented!()) });
    (unsafe { test_value_59(1_f64) });
    (unsafe { test_value_59(unimplemented!()) });
    (unsafe { test_value_59(unimplemented!()) });
}
pub unsafe fn test_float_type_64() {
    (unsafe { test_termination_condition_16(unimplemented!(), -unimplemented!(), None) });
    (unsafe { test_termination_condition_16(unimplemented!(), -unimplemented!(), None) });
    (unsafe { test_termination_condition_16(unimplemented!(), -0_f64, None) });
    (unsafe { test_termination_condition_16(unimplemented!(), 0_f64, None) });
    (unsafe { test_termination_condition_16(unimplemented!(), unimplemented!(), None) });
    (unsafe { test_termination_condition_16(unimplemented!(), unimplemented!(), None) });
    {}
    {}
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_16(_f, -unimplemented!(), Some(_expected))
    });
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_16(_f, unimplemented!(), Some(_expected))
    });
    let mut nan: () = unimplemented!();
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_17(_f, (unsafe { copysign_65(nan, -1_f64) }), Some(_expected))
    });
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_16(_f, nan, Some(_expected))
    });
    {
        (unsafe { test_special_values_60() });
    }
}
pub unsafe fn test_float_type_66() {
    (unsafe { test_termination_condition_17(unimplemented!(), -unimplemented!(), None) });
    (unsafe { test_termination_condition_17(unimplemented!(), -unimplemented!(), None) });
    (unsafe { test_termination_condition_17(unimplemented!(), -0_f64, None) });
    (unsafe { test_termination_condition_17(unimplemented!(), 0_f64, None) });
    (unsafe { test_termination_condition_17(unimplemented!(), unimplemented!(), None) });
    (unsafe { test_termination_condition_17(unimplemented!(), unimplemented!(), None) });
    {
        (unsafe { test_termination_condition_17(unimplemented!(), -unimplemented!(), None) });
        (unsafe { test_termination_condition_17(unimplemented!(), -unimplemented!(), None) });
        (unsafe { test_termination_condition_17(unimplemented!(), unimplemented!(), None) });
        (unsafe { test_termination_condition_17(unimplemented!(), unimplemented!(), None) });
    }
    {}
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_17(_f, -unimplemented!(), Some(_expected))
    });
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_17(_f, unimplemented!(), Some(_expected))
    });
    let mut nan: () = unimplemented!();
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_17(_f, (unsafe { copysign_67(nan, -1_f64) }), Some(_expected))
    });
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_17(_f, nan, Some(_expected))
    });
    {
        (unsafe { test_special_values_61() });
    }
}
pub unsafe fn test_float_type_68() {
    (unsafe { test_termination_condition_18(unimplemented!(), -unimplemented!(), None) });
    (unsafe { test_termination_condition_18(unimplemented!(), -unimplemented!(), None) });
    (unsafe { test_termination_condition_18(unimplemented!(), -0_f64, None) });
    (unsafe { test_termination_condition_18(unimplemented!(), 0_f64, None) });
    (unsafe { test_termination_condition_18(unimplemented!(), unimplemented!(), None) });
    (unsafe { test_termination_condition_18(unimplemented!(), unimplemented!(), None) });
    {
        (unsafe { test_termination_condition_18(unimplemented!(), -unimplemented!(), None) });
        (unsafe { test_termination_condition_18(unimplemented!(), -unimplemented!(), None) });
        (unsafe { test_termination_condition_18(unimplemented!(), unimplemented!(), None) });
        (unsafe { test_termination_condition_18(unimplemented!(), unimplemented!(), None) });
    }
    {
        (unsafe { test_termination_condition_18(unimplemented!(), -unimplemented!(), None) });
        (unsafe { test_termination_condition_18(unimplemented!(), -unimplemented!(), None) });
        (unsafe { test_termination_condition_18(unimplemented!(), unimplemented!(), None) });
        (unsafe { test_termination_condition_18(unimplemented!(), unimplemented!(), None) });
    }
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_18(_f, -unimplemented!(), Some(_expected))
    });
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_18(_f, unimplemented!(), Some(_expected))
    });
    let mut nan: () = unimplemented!();
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_18(_f, (unsafe { copysign_69(nan, -1_f64) }), Some(_expected))
    });
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_18(_f, nan, Some(_expected))
    });
    {}
}
pub unsafe fn test_float_type_70() {
    (unsafe { test_termination_condition_19(unimplemented!(), -unimplemented!(), None) });
    (unsafe { test_termination_condition_19(unimplemented!(), -unimplemented!(), None) });
    (unsafe { test_termination_condition_19(unimplemented!(), -0_f64, None) });
    (unsafe { test_termination_condition_19(unimplemented!(), 0_f64, None) });
    (unsafe { test_termination_condition_19(unimplemented!(), unimplemented!(), None) });
    (unsafe { test_termination_condition_19(unimplemented!(), unimplemented!(), None) });
    {}
    {}
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_19(_f, -unimplemented!(), Some(_expected))
    });
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_19(_f, unimplemented!(), Some(_expected))
    });
    let mut nan: () = unimplemented!();
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_20(_f, (unsafe { copysign_65(nan, -1_f64) }), Some(_expected))
    });
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_19(_f, nan, Some(_expected))
    });
    {
        (unsafe { test_special_values_62() });
    }
}
pub unsafe fn test_float_type_71() {
    (unsafe { test_termination_condition_20(unimplemented!(), -unimplemented!(), None) });
    (unsafe { test_termination_condition_20(unimplemented!(), -unimplemented!(), None) });
    (unsafe { test_termination_condition_20(unimplemented!(), -0_f64, None) });
    (unsafe { test_termination_condition_20(unimplemented!(), 0_f64, None) });
    (unsafe { test_termination_condition_20(unimplemented!(), unimplemented!(), None) });
    (unsafe { test_termination_condition_20(unimplemented!(), unimplemented!(), None) });
    {
        (unsafe { test_termination_condition_20(unimplemented!(), -unimplemented!(), None) });
        (unsafe { test_termination_condition_20(unimplemented!(), -unimplemented!(), None) });
        (unsafe { test_termination_condition_20(unimplemented!(), unimplemented!(), None) });
        (unsafe { test_termination_condition_20(unimplemented!(), unimplemented!(), None) });
    }
    {}
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_20(_f, -unimplemented!(), Some(_expected))
    });
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_20(_f, unimplemented!(), Some(_expected))
    });
    let mut nan: () = unimplemented!();
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_20(_f, (unsafe { copysign_67(nan, -1_f64) }), Some(_expected))
    });
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_20(_f, nan, Some(_expected))
    });
    {
        (unsafe { test_special_values_63() });
    }
}
pub unsafe fn test_float_type_72() {
    (unsafe { test_termination_condition_21(unimplemented!(), -unimplemented!(), None) });
    (unsafe { test_termination_condition_21(unimplemented!(), -unimplemented!(), None) });
    (unsafe { test_termination_condition_21(unimplemented!(), -0_f64, None) });
    (unsafe { test_termination_condition_21(unimplemented!(), 0_f64, None) });
    (unsafe { test_termination_condition_21(unimplemented!(), unimplemented!(), None) });
    (unsafe { test_termination_condition_21(unimplemented!(), unimplemented!(), None) });
    {
        (unsafe { test_termination_condition_21(unimplemented!(), -unimplemented!(), None) });
        (unsafe { test_termination_condition_21(unimplemented!(), -unimplemented!(), None) });
        (unsafe { test_termination_condition_21(unimplemented!(), unimplemented!(), None) });
        (unsafe { test_termination_condition_21(unimplemented!(), unimplemented!(), None) });
    }
    {
        (unsafe { test_termination_condition_21(unimplemented!(), -unimplemented!(), None) });
        (unsafe { test_termination_condition_21(unimplemented!(), -unimplemented!(), None) });
        (unsafe { test_termination_condition_21(unimplemented!(), unimplemented!(), None) });
        (unsafe { test_termination_condition_21(unimplemented!(), unimplemented!(), None) });
    }
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_21(_f, -unimplemented!(), Some(_expected))
    });
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_21(_f, unimplemented!(), Some(_expected))
    });
    let mut nan: () = unimplemented!();
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_21(_f, (unsafe { copysign_69(nan, -1_f64) }), Some(_expected))
    });
    (unsafe {
        let _f: () = unimplemented!();
        let _expected: () = unimplemented!();
        test_termination_condition_21(_f, nan, Some(_expected))
    });
    {}
}
pub unsafe fn test_all_float_types_73() {
    (unsafe { test_float_type_64() });
    (unsafe { test_float_type_66() });
    (unsafe { test_float_type_68() });
}
pub unsafe fn test_all_float_types_74() {
    (unsafe { test_float_type_70() });
    (unsafe { test_float_type_71() });
    (unsafe { test_float_type_72() });
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
    (unsafe { test_all_float_types_73() });
    (unsafe { test_all_float_types_74() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
