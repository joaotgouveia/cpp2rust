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
    let mut format_ctx: () =
        (unsafe { test_format_context_create_0(out.clone(), unimplemented!()) });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_5(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () =
        (unsafe { test_format_context_create_0(out.clone(), unimplemented!()) });
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
    let mut format_ctx: () =
        (unsafe { test_format_context_create_0(out.clone(), unimplemented!()) });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_7(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () =
        (unsafe { test_format_context_create_0(out.clone(), unimplemented!()) });
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
    let mut format_ctx: () =
        (unsafe { test_format_context_create_0(out.clone(), unimplemented!()) });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_9(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () =
        (unsafe { test_format_context_create_0(out.clone(), unimplemented!()) });
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
    let mut format_ctx: () =
        (unsafe { test_format_context_create_2(out.clone(), unimplemented!()) });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_11(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () =
        (unsafe { test_format_context_create_2(out.clone(), unimplemented!()) });
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
    let mut format_ctx: () =
        (unsafe { test_format_context_create_2(out.clone(), unimplemented!()) });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_13(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () =
        (unsafe { test_format_context_create_2(out.clone(), unimplemented!()) });
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
    let mut format_ctx: () =
        (unsafe { test_format_context_create_2(out.clone(), unimplemented!()) });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_15(mut expected: (), mut fmt: (), mut arg: (), mut offset: ()) {
    let mut parse_ctx: () = unimplemented!();
    let mut formatter: () = unimplemented!();
    const _: () = assert!(true, "std::semiregular<decltype(formatter)>");;
    let mut it: () = unimplemented!();
    unimplemented!();
    let mut result: () = unimplemented!();
    let mut out: () = unimplemented!();
    let mut format_ctx: () =
        (unsafe { test_format_context_create_2(out.clone(), unimplemented!()) });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_termination_condition_16(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_4(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_4(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_17(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_5(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_5(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_18(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_6(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_6(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_19(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_7(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_7(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_20(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_8(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_8(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_21(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_9(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_9(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_22(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_10(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_10(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_23(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_11(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_11(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_24(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_12(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_12(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_25(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_13(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_13(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_26(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_14(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_14(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_termination_condition_27(mut expected: (), mut f: (), mut arg: ()) {
    let mut fmt: () = unimplemented!();
    unimplemented!();
    (unsafe { test_15(unimplemented!(), unimplemented!(), arg, 1_()) });
    unimplemented!();
    (unsafe { test_15(unimplemented!(), unimplemented!(), arg, 0_()) });
}
pub unsafe fn test_signed_integral_type_28() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_16(_expected, _f, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_16(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_16(_expected, _f, 127_())
    });
    {}
    {}
    {}
    {}
}
pub unsafe fn test_signed_integral_type_29() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_17(_expected, _f, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_17(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_17(_expected, _f, 127_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_17(_expected, _f, -32768_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_17(_expected, _f, 32767_())
        });
    }
    {}
    {}
    {}
}
pub unsafe fn test_signed_integral_type_30() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_18(_expected, _f, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_18(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_18(_expected, _f, 127_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_18(_expected, _f, -32768_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_18(_expected, _f, 32767_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_18(_expected, _f, -2147483648_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_18(_expected, _f, 2147483647_())
        });
    }
    {}
    {}
}
pub unsafe fn test_signed_integral_type_31() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_19(_expected, _f, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_19(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_19(_expected, _f, 127_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_19(_expected, _f, -32768_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_19(_expected, _f, 32767_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_19(_expected, _f, -2147483648_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_19(_expected, _f, 2147483647_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_19(_expected, _f, unimplemented!())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_19(_expected, _f, unimplemented!())
        });
    }
    {}
}
pub unsafe fn test_signed_integral_type_32() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_20(_expected, _f, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_20(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_20(_expected, _f, 127_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_20(_expected, _f, -32768_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_20(_expected, _f, 32767_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_20(_expected, _f, -2147483648_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_20(_expected, _f, 2147483647_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_20(_expected, _f, unimplemented!())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_20(_expected, _f, unimplemented!())
        });
    }
    {}
}
pub unsafe fn test_signed_integral_type_33() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_21(_expected, _f, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_21(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_21(_expected, _f, 127_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_21(_expected, _f, -32768_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_21(_expected, _f, 32767_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_21(_expected, _f, -2147483648_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_21(_expected, _f, 2147483647_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_21(_expected, _f, unimplemented!())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_21(_expected, _f, unimplemented!())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_21(_expected, _f, unimplemented!())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_21(_expected, _f, unimplemented!())
        });
    }
}
pub unsafe fn test_signed_integral_type_34() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_22(_expected, _f, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_22(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_22(_expected, _f, 127_())
    });
    {}
    {}
    {}
    {}
}
pub unsafe fn test_signed_integral_type_35() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_23(_expected, _f, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_23(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_23(_expected, _f, 127_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_23(_expected, _f, -32768_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_23(_expected, _f, 32767_())
        });
    }
    {}
    {}
    {}
}
pub unsafe fn test_signed_integral_type_36() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_24(_expected, _f, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_24(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_24(_expected, _f, 127_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_24(_expected, _f, -32768_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_24(_expected, _f, 32767_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_24(_expected, _f, -2147483648_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_24(_expected, _f, 2147483647_())
        });
    }
    {}
    {}
}
pub unsafe fn test_signed_integral_type_37() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_25(_expected, _f, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_25(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_25(_expected, _f, 127_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_25(_expected, _f, -32768_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_25(_expected, _f, 32767_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_25(_expected, _f, -2147483648_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_25(_expected, _f, 2147483647_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_25(_expected, _f, unimplemented!())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_25(_expected, _f, unimplemented!())
        });
    }
    {}
}
pub unsafe fn test_signed_integral_type_38() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_26(_expected, _f, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_26(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_26(_expected, _f, 127_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_26(_expected, _f, -32768_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_26(_expected, _f, 32767_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_26(_expected, _f, -2147483648_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_26(_expected, _f, 2147483647_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_26(_expected, _f, unimplemented!())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_26(_expected, _f, unimplemented!())
        });
    }
    {}
}
pub unsafe fn test_signed_integral_type_39() {
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_27(_expected, _f, -128_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_27(_expected, _f, 0_())
    });
    (unsafe {
        let _expected: () = unimplemented!();
        let _f: () = unimplemented!();
        test_termination_condition_27(_expected, _f, 127_())
    });
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_27(_expected, _f, -32768_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_27(_expected, _f, 32767_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_27(_expected, _f, -2147483648_())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_27(_expected, _f, 2147483647_())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_27(_expected, _f, unimplemented!())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_27(_expected, _f, unimplemented!())
        });
    }
    {
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_27(_expected, _f, unimplemented!())
        });
        (unsafe {
            let _expected: () = unimplemented!();
            let _f: () = unimplemented!();
            test_termination_condition_27(_expected, _f, unimplemented!())
        });
    }
}
pub unsafe fn test_all_signed_integral_types_40() {
    (unsafe { test_signed_integral_type_28() });
    (unsafe { test_signed_integral_type_29() });
    (unsafe { test_signed_integral_type_30() });
    (unsafe { test_signed_integral_type_31() });
    (unsafe { test_signed_integral_type_32() });
    (unsafe { test_signed_integral_type_33() });
}
pub unsafe fn test_all_signed_integral_types_41() {
    (unsafe { test_signed_integral_type_34() });
    (unsafe { test_signed_integral_type_35() });
    (unsafe { test_signed_integral_type_36() });
    (unsafe { test_signed_integral_type_37() });
    (unsafe { test_signed_integral_type_38() });
    (unsafe { test_signed_integral_type_39() });
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
    (unsafe { test_all_signed_integral_types_40() });
    (unsafe { test_all_signed_integral_types_41() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
