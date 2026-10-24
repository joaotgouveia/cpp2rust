extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut dummy_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 42_() });
pub unsafe fn return_value_1(mut _a0: ()) -> () {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0));
}
pub unsafe fn return_lvalue_2(mut _a0: ()) -> *mut () {
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0));
}
pub unsafe fn return_const_lvalue_3(mut _a0: ()) -> *const () {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0));
}
pub unsafe fn return_rvalue_4(mut _a0: ()) -> *mut () {
    return &mut *(&raw mut (*unimplemented!())).cast::<()>();
}
pub unsafe fn return_const_rvalue_5(mut _a0: ()) -> *const () {
    return &*(&raw const (*unimplemented!())).cast::<()>();
}
pub unsafe fn CheckCall_6() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_7() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_8() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_9() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_10() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_11() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_12() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_13() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_14() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_15() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_16() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_17() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_18() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_19() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_20() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_21() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_22() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_23() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_24() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_25() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_26() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_27() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_28() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_29() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_30() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_31() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_32() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_33() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_34() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_35() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_36() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_37() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_38() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_39() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_40() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_41() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_42() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_43() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_44() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_45() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_46() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_47() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_48() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_49() -> () {
    return unimplemented!();
}
pub unsafe fn CheckCall_50() -> () {
    return unimplemented!();
}
pub unsafe fn do_test_51(mut func: Option<unsafe fn(()) -> ()>) {
    let mut ret: std___bind_int__ptrref__int___const_std_placeholders___ph_1__ref_ =
        unimplemented!();
    let mut ret_r: std___bind_r_int__int__ptrref__int___const_std_placeholders___ph_1__ref_ = (unsafe {
        bind_52(
            &mut func,
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut _1_53)),
        )
    });
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let mut exp: () = ((unsafe {
        let mut ___args: () = 100_();
        ret(&mut ___args)
    }) as ());
    let mut exp2: () = ((unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret(&mut ___args_0, &mut ___args_1)
    }) as ());
    let mut exp_r: () = (unsafe {
        let mut ___args: () = 100_();
        ret_r(&mut ___args)
    });
    let mut exp_r2: () = (unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret_r(&mut ___args_0, &mut ___args_1)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {}
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "!CheckCall<BindR>()");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub unsafe fn do_test_54(mut func: Option<unsafe fn(()) -> *mut ()>) {
    let mut ret: std___bind_int_ref_ptrref__int___const_std_placeholders___ph_1__ref_ =
        unimplemented!();
    let mut ret_r: std___bind_r_int_ref__int_ref_ptrref__int___const_std_placeholders___ph_1__ref_ =
        (unsafe {
            bind_55(
                &mut func,
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut _1_53)),
            )
        });
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: *mut () = &mut (unsafe {
        let mut ___args: () = 100_();
        ret(&mut ___args)
    });
    let exp2: *mut () = &mut (unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret(&mut ___args_0, &mut ___args_1)
    });
    let exp_r: *mut () = &mut (unsafe {
        let mut ___args: () = 100_();
        ret_r(&mut ___args)
    });
    let exp_r2: *mut () = &mut (unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret_r(&mut ___args_0, &mut ___args_1)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "!CheckCall<BindR>()");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub unsafe fn do_test_56(mut func: Option<unsafe fn(()) -> *const ()>) {
    let mut ret: std___bind_const_int_ref_ptrref__int___const_std_placeholders___ph_1__ref_ =
        unimplemented!();
    let mut ret_r : std___bind_r_const_int_ref__const_int_ref_ptrref__int___const_std_placeholders___ph_1__ref_ = ( unsafe { bind_57 ( & mut func , & (*std::cell::LazyCell::force_mut(&mut *&raw mut _1_53)) , ) } )  ;
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: *const () = &(unsafe {
        let mut ___args: () = 100_();
        ret(&mut ___args)
    });
    let exp2: *const () = &(unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret(&mut ___args_0, &mut ___args_1)
    });
    let exp_r: *const () = &(unsafe {
        let mut ___args: () = 100_();
        ret_r(&mut ___args)
    });
    let exp_r2: *const () = &(unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret_r(&mut ___args_0, &mut ___args_1)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "!CheckCall<BindR>()");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub unsafe fn do_test_58(mut func: Option<unsafe fn(()) -> *mut ()>) {
    let mut ret: std___bind_int_refref_ptrref__int___const_std_placeholders___ph_1__ref_ =
        unimplemented!();
    let mut ret_r : std___bind_r_int_refref__int_refref_ptrref__int___const_std_placeholders___ph_1__ref_ = ( unsafe { bind_59 ( & mut func , & (*std::cell::LazyCell::force_mut(&mut *&raw mut _1_53)) , ) } )  ;
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: *mut () = &mut (unsafe {
        let mut ___args: () = 100_();
        ret(&mut ___args)
    });
    let exp2: *mut () = &mut (unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret(&mut ___args_0, &mut ___args_1)
    });
    let exp_r: *mut () = &mut (unsafe {
        let mut ___args: () = 100_();
        ret_r(&mut ___args)
    });
    let exp_r2: *mut () = &mut (unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret_r(&mut ___args_0, &mut ___args_1)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "!CheckCall<BindR>()");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub unsafe fn do_test_60(mut func: Option<unsafe fn(()) -> *const ()>) {
    let mut ret: std___bind_const_int_refref_ptrref__int___const_std_placeholders___ph_1__ref_ =
        unimplemented!();
    let mut ret_r : std___bind_r_const_int_refref__const_int_refref_ptrref__int___const_std_placeholders___ph_1__ref_ = ( unsafe { bind_61 ( & mut func , & (*std::cell::LazyCell::force_mut(&mut *&raw mut _1_53)) , ) } )  ;
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: *const () = &(unsafe {
        let mut ___args: () = 100_();
        ret(&mut ___args)
    });
    let exp2: *const () = &(unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret(&mut ___args_0, &mut ___args_1)
    });
    let exp_r: *const () = &(unsafe {
        let mut ___args: () = 100_();
        ret_r(&mut ___args)
    });
    let exp_r2: *const () = &(unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret_r(&mut ___args_0, &mut ___args_1)
    });
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "!CheckCall<BindR>()");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub unsafe fn do_test_r_62(mut func: Option<unsafe fn(()) -> ()>) {
    let mut ret: std___bind_r_long__int__ptrref__int___const_std_placeholders___ph_1__ref_ = (unsafe {
        bind_63(
            &mut func,
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut _1_53)),
        )
    });
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = (unsafe {
        let mut ___args: () = 100_();
        ret(&mut ___args)
    });
    let mut exp2: () = (unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret(&mut ___args_0, &mut ___args_1)
    });
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub unsafe fn do_test_r_64(mut func: Option<unsafe fn(()) -> *mut ()>) {
    let mut ret: std___bind_r_long__int_ref_ptrref__int___const_std_placeholders___ph_1__ref_ = (unsafe {
        bind_65(
            &mut func,
            &(*std::cell::LazyCell::force_mut(&mut *&raw mut _1_53)),
        )
    });
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = (unsafe {
        let mut ___args: () = 100_();
        ret(&mut ___args)
    });
    let mut exp2: () = (unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret(&mut ___args_0, &mut ___args_1)
    });
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub unsafe fn do_test_r_66(mut func: Option<unsafe fn(()) -> *const ()>) {
    let mut ret : std___bind_r_long__const_int_ref_ptrref__int___const_std_placeholders___ph_1__ref_ = ( unsafe { bind_67 ( & mut func , & (*std::cell::LazyCell::force_mut(&mut *&raw mut _1_53)) , ) } )  ;
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = (unsafe {
        let mut ___args: () = 100_();
        ret(&mut ___args)
    });
    let mut exp2: () = (unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret(&mut ___args_0, &mut ___args_1)
    });
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub unsafe fn do_test_r_68(mut func: Option<unsafe fn(()) -> *mut ()>) {
    let mut ret: std___bind_r_long__int_refref_ptrref__int___const_std_placeholders___ph_1__ref_ =
        (unsafe {
            bind_69(
                &mut func,
                &(*std::cell::LazyCell::force_mut(&mut *&raw mut _1_53)),
            )
        });
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = (unsafe {
        let mut ___args: () = 100_();
        ret(&mut ___args)
    });
    let mut exp2: () = (unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret(&mut ___args_0, &mut ___args_1)
    });
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub unsafe fn do_test_r_70(mut func: Option<unsafe fn(()) -> *const ()>) {
    let mut ret : std___bind_r_long__const_int_refref_ptrref__int___const_std_placeholders___ph_1__ref_ = ( unsafe { bind_71 ( & mut func , & (*std::cell::LazyCell::force_mut(&mut *&raw mut _1_53)) , ) } )  ;
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: () = (unsafe {
        let mut ___args: () = 100_();
        ret(&mut ___args)
    });
    let mut exp2: () = (unsafe {
        let mut ___args_0: () = 101_();
        let mut ___args_1: () = 102_();
        ret(&mut ___args_0, &mut ___args_1)
    });
    unimplemented!();
    unimplemented!();
    {
        const _: () = assert!(true, "!CheckCall<Bind>()");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
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
    (unsafe { do_test_51(Some(return_value_1)) });
    (unsafe { do_test_54(Some(return_lvalue_2)) });
    (unsafe { do_test_56(Some(return_const_lvalue_3)) });
    (unsafe { do_test_58(Some(return_rvalue_4)) });
    (unsafe { do_test_60(Some(return_const_rvalue_5)) });
    (unsafe { do_test_r_62(Some(return_value_1)) });
    (unsafe { do_test_r_64(Some(return_lvalue_2)) });
    (unsafe { do_test_r_66(Some(return_const_lvalue_3)) });
    (unsafe { do_test_r_68(Some(return_rvalue_4)) });
    (unsafe { do_test_r_70(Some(return_const_rvalue_5)) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const dummy_0);
}
