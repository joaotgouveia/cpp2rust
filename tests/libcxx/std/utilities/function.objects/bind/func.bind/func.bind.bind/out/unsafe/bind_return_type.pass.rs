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
    let mut ret: () = unimplemented!();
    let mut ret_r: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let mut exp: () = (unimplemented!() as ());
    let mut exp2: () = (unimplemented!() as ());
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
pub unsafe fn do_test_52(mut func: Option<unsafe fn(()) -> *mut ()>) {
    let mut ret: () = unimplemented!();
    let mut ret_r: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: *mut () = &mut (*unimplemented!());
    let exp2: *mut () = &mut (*unimplemented!());
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
pub unsafe fn do_test_53(mut func: Option<unsafe fn(()) -> *const ()>) {
    let mut ret: () = unimplemented!();
    let mut ret_r: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: *const () = &(*unimplemented!());
    let exp2: *const () = &(*unimplemented!());
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
pub unsafe fn do_test_54(mut func: Option<unsafe fn(()) -> *mut ()>) {
    let mut ret: () = unimplemented!();
    let mut ret_r: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: *mut () = &mut (*unimplemented!());
    let exp2: *mut () = &mut (*unimplemented!());
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
pub unsafe fn do_test_55(mut func: Option<unsafe fn(()) -> *const ()>) {
    let mut ret: () = unimplemented!();
    let mut ret_r: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: *const () = &(*unimplemented!());
    let exp2: *const () = &(*unimplemented!());
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
pub unsafe fn do_test_r_56(mut func: Option<unsafe fn(()) -> ()>) {
    let mut ret: () = unimplemented!();
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
pub unsafe fn do_test_r_57(mut func: Option<unsafe fn(()) -> *mut ()>) {
    let mut ret: () = unimplemented!();
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
pub unsafe fn do_test_r_58(mut func: Option<unsafe fn(()) -> *const ()>) {
    let mut ret: () = unimplemented!();
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
pub unsafe fn do_test_r_59(mut func: Option<unsafe fn(()) -> *mut ()>) {
    let mut ret: () = unimplemented!();
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
pub unsafe fn do_test_r_60(mut func: Option<unsafe fn(()) -> *const ()>) {
    let mut ret: () = unimplemented!();
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
    (unsafe { do_test_52(Some(return_lvalue_2)) });
    (unsafe { do_test_53(Some(return_const_lvalue_3)) });
    (unsafe { do_test_54(Some(return_rvalue_4)) });
    (unsafe { do_test_55(Some(return_const_rvalue_5)) });
    (unsafe { do_test_r_56(Some(return_value_1)) });
    (unsafe { do_test_r_57(Some(return_lvalue_2)) });
    (unsafe { do_test_r_58(Some(return_const_lvalue_3)) });
    (unsafe { do_test_r_59(Some(return_rvalue_4)) });
    (unsafe { do_test_r_60(Some(return_const_rvalue_5)) });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const dummy_0);
}
