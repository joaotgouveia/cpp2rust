extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut dummy_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 42 });
pub unsafe fn return_value_1(mut _a0: i32) -> i32 {
    return (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0));
}
pub unsafe fn return_lvalue_2(mut _a0: i32) -> *mut i32 {
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0));
}
pub unsafe fn return_const_lvalue_3(mut _a0: i32) -> *const i32 {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0));
}
pub unsafe fn return_rvalue_4(mut _a0: i32) -> *mut i32 {
    return &mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0));
}
pub unsafe fn return_const_rvalue_5(mut _a0: i32) -> *const i32 {
    return &(*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0));
}
pub unsafe fn CheckCall_6() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_7() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_8() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_9() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_10() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_11() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_12() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_13() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_14() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_15() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_16() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_17() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_18() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_19() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_20() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_21() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_22() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_23() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_24() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_25() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_26() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_27() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_28() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_29() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_30() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_31() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_32() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_33() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_34() -> bool {
    return unimplemented!();
}
pub unsafe fn CheckCall_35() -> bool {
    return unimplemented!();
}
pub unsafe fn do_test_36(mut func: Option<unsafe fn(i32) -> i32>) {
    let mut ret: () = unimplemented!();
    let mut ret_r: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let mut exp: i32 = (unsafe {
        let mut ___args: i32 = 100;
        ret(&mut ___args)
    });
    let mut exp2: i32 = (unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret(&mut ___args_0, &mut ___args_1)
    });
    let mut exp_r: i32 = (unsafe {
        let mut ___args: i32 = 100;
        ret_r(&mut ___args)
    });
    let mut exp_r2: i32 = (unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret_r(&mut ___args_0, &mut ___args_1)
    });
    assert!(((exp) == (42)));
    assert!(((exp2) == (42)));
    assert!(((exp_r) == (42)));
    assert!(((exp_r2) == (42)));
    {}
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub unsafe fn do_test_37(mut func: Option<unsafe fn(i32) -> *mut i32>) {
    let mut ret: () = unimplemented!();
    let mut ret_r: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: *mut i32 = &mut (unsafe {
        let mut ___args: i32 = 100;
        ret(&mut ___args)
    });
    let exp2: *mut i32 = &mut (unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret(&mut ___args_0, &mut ___args_1)
    });
    let exp_r: *mut i32 = &mut (unsafe {
        let mut ___args: i32 = 100;
        ret_r(&mut ___args)
    });
    let exp_r2: *mut i32 = &mut (unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret_r(&mut ___args_0, &mut ___args_1)
    });
    assert!(((*exp) == (42)));
    assert!(((*exp2) == (42)));
    assert!(((*exp_r) == (42)));
    assert!(((*exp_r2) == (42)));
    {
        assert!(
            ((exp)
                == (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32))
        );
        assert!(
            ((exp2)
                == (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32))
        );
        assert!(
            ((exp_r)
                == (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32))
        );
        assert!(
            ((exp_r2)
                == (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32))
        );
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub unsafe fn do_test_38(mut func: Option<unsafe fn(i32) -> *const i32>) {
    let mut ret: () = unimplemented!();
    let mut ret_r: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: *const i32 = &(unsafe {
        let mut ___args: i32 = 100;
        ret(&mut ___args)
    });
    let exp2: *const i32 = &(unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret(&mut ___args_0, &mut ___args_1)
    });
    let exp_r: *const i32 = &(unsafe {
        let mut ___args: i32 = 100;
        ret_r(&mut ___args)
    });
    let exp_r2: *const i32 = &(unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret_r(&mut ___args_0, &mut ___args_1)
    });
    assert!(((*exp) == (42)));
    assert!(((*exp2) == (42)));
    assert!(((*exp_r) == (42)));
    assert!(((*exp_r2) == (42)));
    {
        assert!(
            ((exp)
                == ((&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32)
                    .cast_const()))
        );
        assert!(
            ((exp2)
                == ((&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32)
                    .cast_const()))
        );
        assert!(
            ((exp_r)
                == ((&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32)
                    .cast_const()))
        );
        assert!(
            ((exp_r2)
                == ((&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32)
                    .cast_const()))
        );
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub unsafe fn do_test_39(mut func: Option<unsafe fn(i32) -> *mut i32>) {
    let mut ret: () = unimplemented!();
    let mut ret_r: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: *mut i32 = &mut (unsafe {
        let mut ___args: i32 = 100;
        ret(&mut ___args)
    });
    let exp2: *mut i32 = &mut (unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret(&mut ___args_0, &mut ___args_1)
    });
    let exp_r: *mut i32 = &mut (unsafe {
        let mut ___args: i32 = 100;
        ret_r(&mut ___args)
    });
    let exp_r2: *mut i32 = &mut (unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret_r(&mut ___args_0, &mut ___args_1)
    });
    assert!(((*exp) == (42)));
    assert!(((*exp2) == (42)));
    assert!(((*exp_r) == (42)));
    assert!(((*exp_r2) == (42)));
    {
        assert!(
            ((exp)
                == (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32))
        );
        assert!(
            ((exp2)
                == (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32))
        );
        assert!(
            ((exp_r)
                == (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32))
        );
        assert!(
            ((exp_r2)
                == (&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32))
        );
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub unsafe fn do_test_40(mut func: Option<unsafe fn(i32) -> *const i32>) {
    let mut ret: () = unimplemented!();
    let mut ret_r: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR, Expect>::value");;
    const _: () = assert!(true, "std::is_same<RetR2, Expect>::value");;
    let exp: *const i32 = &(unsafe {
        let mut ___args: i32 = 100;
        ret(&mut ___args)
    });
    let exp2: *const i32 = &(unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret(&mut ___args_0, &mut ___args_1)
    });
    let exp_r: *const i32 = &(unsafe {
        let mut ___args: i32 = 100;
        ret_r(&mut ___args)
    });
    let exp_r2: *const i32 = &(unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret_r(&mut ___args_0, &mut ___args_1)
    });
    assert!(((*exp) == (42)));
    assert!(((*exp2) == (42)));
    assert!(((*exp_r) == (42)));
    assert!(((*exp_r2) == (42)));
    {
        assert!(
            ((exp)
                == ((&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32)
                    .cast_const()))
        );
        assert!(
            ((exp2)
                == ((&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32)
                    .cast_const()))
        );
        assert!(
            ((exp_r)
                == ((&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32)
                    .cast_const()))
        );
        assert!(
            ((exp_r2)
                == ((&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut dummy_0))
                    as *mut i32)
                    .cast_const()))
        );
    }
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<BindR, int>()");;
        const _: () = assert!(true, "CheckCall<BindR, int, int>()");;
    }
}
pub unsafe fn do_test_r_41(mut func: Option<unsafe fn(i32) -> i32>) {
    let mut ret: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = (unsafe {
        let mut ___args: i32 = 100;
        ret(&mut ___args)
    });
    let mut exp2: i64 = (unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret(&mut ___args_0, &mut ___args_1)
    });
    assert!(((exp) == (42_i64)));
    assert!(((exp2) == (42_i64)));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub unsafe fn do_test_r_42(mut func: Option<unsafe fn(i32) -> *mut i32>) {
    let mut ret: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = (unsafe {
        let mut ___args: i32 = 100;
        ret(&mut ___args)
    });
    let mut exp2: i64 = (unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret(&mut ___args_0, &mut ___args_1)
    });
    assert!(((exp) == (42_i64)));
    assert!(((exp2) == (42_i64)));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub unsafe fn do_test_r_43(mut func: Option<unsafe fn(i32) -> *const i32>) {
    let mut ret: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = (unsafe {
        let mut ___args: i32 = 100;
        ret(&mut ___args)
    });
    let mut exp2: i64 = (unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret(&mut ___args_0, &mut ___args_1)
    });
    assert!(((exp) == (42_i64)));
    assert!(((exp2) == (42_i64)));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub unsafe fn do_test_r_44(mut func: Option<unsafe fn(i32) -> *mut i32>) {
    let mut ret: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = (unsafe {
        let mut ___args: i32 = 100;
        ret(&mut ___args)
    });
    let mut exp2: i64 = (unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret(&mut ___args_0, &mut ___args_1)
    });
    assert!(((exp) == (42_i64)));
    assert!(((exp2) == (42_i64)));
    {
        const _: () = assert!(true, "");;
        const _: () = assert!(true, "CheckCall<Bind, int>()");;
        const _: () = assert!(true, "CheckCall<Bind, int, int>()");;
    }
}
pub unsafe fn do_test_r_45(mut func: Option<unsafe fn(i32) -> *const i32>) {
    let mut ret: () = unimplemented!();
    const _: () = assert!(true, "std::is_same<Ret, Expect>::value");;
    const _: () = assert!(true, "std::is_same<Ret2, Expect>::value");;
    let mut exp: i64 = (unsafe {
        let mut ___args: i32 = 100;
        ret(&mut ___args)
    });
    let mut exp2: i64 = (unsafe {
        let mut ___args_0: i32 = 101;
        let mut ___args_1: i32 = 102;
        ret(&mut ___args_0, &mut ___args_1)
    });
    assert!(((exp) == (42_i64)));
    assert!(((exp2) == (42_i64)));
    {
        const _: () = assert!(true, "");;
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { do_test_36(Some(return_value_1)) });
    (unsafe { do_test_37(Some(return_lvalue_2)) });
    (unsafe { do_test_38(Some(return_const_lvalue_3)) });
    (unsafe { do_test_39(Some(return_rvalue_4)) });
    (unsafe { do_test_40(Some(return_const_rvalue_5)) });
    (unsafe { do_test_r_41(Some(return_value_1)) });
    (unsafe { do_test_r_42(Some(return_lvalue_2)) });
    (unsafe { do_test_r_43(Some(return_const_lvalue_3)) });
    (unsafe { do_test_r_44(Some(return_rvalue_4)) });
    (unsafe { do_test_r_45(Some(return_const_rvalue_5)) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const dummy_0);
}
