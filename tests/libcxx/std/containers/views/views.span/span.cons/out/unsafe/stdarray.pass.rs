extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn checkCV_0() {
    let mut arr: Vec<i32> = Vec::from([1, 2, 3]);
    {
        let mut s1: () = unimplemented!();
    }
    {
        let mut s1: () = unimplemented!();
    }
    {
        let mut s1: () = unimplemented!();
        let mut s2: () = unimplemented!();
        let mut s3: () = unimplemented!();
        let mut s4: () = unimplemented!();
    }
    {
        let mut s1: () = unimplemented!();
        let mut s2: () = unimplemented!();
        let mut s3: () = unimplemented!();
        let mut s4: () = unimplemented!();
    }
}
pub unsafe fn testConstructorArray_1() -> bool {
    let mut val: Vec<i32> = Vec::from([0_i32, 0_i32]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == (&mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *mut i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == (&mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i32> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *mut i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_2() -> bool {
    let mut val: Vec<i32> = Vec::from([0_i32, 0_i32]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == (&(*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *const i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == (&(*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i32> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *const i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_3() -> bool {
    let mut val: Vec<i32> = Vec::from([0_i32, 0_i32]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == ((&mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i32> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *mut i32)
            .cast_const()))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == ((&mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i32> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *mut i32)
                .cast_const())))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_4() -> bool {
    let mut val: Vec<i64> = Vec::from([0_i64, 0_i64]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == (&mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i64> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *mut i64))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == (&mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i64> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *mut i64)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_5() -> bool {
    let mut val: Vec<i64> = Vec::from([0_i64, 0_i64]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == (&(*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i64> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *const i64))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == (&(*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i64> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *const i64)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_6() -> bool {
    let mut val: Vec<i64> = Vec::from([0_i64, 0_i64]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == ((&mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<i64> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *mut i64)
            .cast_const()))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == ((&mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i64> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *mut i64)
                .cast_const())))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_7() -> bool {
    let mut val: Vec<f64> = Vec::from([0_f64, 0_f64]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == (&mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<f64> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *mut f64))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == (&mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<f64> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *mut f64)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_8() -> bool {
    let mut val: Vec<f64> = Vec::from([0_f64, 0_f64]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == (&(*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<f64> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *const f64))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == (&(*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<f64> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *const f64)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_9() -> bool {
    let mut val: Vec<f64> = Vec::from([0_f64, 0_f64]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == ((&mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<f64> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *mut f64)
            .cast_const()))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == ((&mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<f64> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *mut f64)
                .cast_const())))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_10() -> bool {
    let mut val: Vec<A> = Vec::from([<A>::default(), <A>::default()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == (&mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<A> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *mut A))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == (&mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<A> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *mut A)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_11() -> bool {
    let mut val: Vec<A> = Vec::from([<A>::default(), <A>::default()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == (&(*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<A> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *const A))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == (&(*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<A> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *const A)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_12() -> bool {
    let mut val: Vec<A> = Vec::from([<A>::default(), <A>::default()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == ((&mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<A> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *mut A)
            .cast_const()))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == ((&mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<A> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *mut A)
                .cast_const())))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_13() -> bool {
    let mut val: Vec<*mut i32> = Vec::from([std::ptr::null_mut(), std::ptr::null_mut()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == (&mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<*mut i32> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *mut *mut i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == (&mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<*mut i32> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *mut *mut i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_14() -> bool {
    let mut val: Vec<*mut i32> = Vec::from([std::ptr::null_mut(), std::ptr::null_mut()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == (&(*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<*mut i32> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *const *mut i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == (&(*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<*mut i32> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *const *mut i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_15() -> bool {
    let mut val: Vec<*mut i32> = Vec::from([std::ptr::null_mut(), std::ptr::null_mut()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == ((&mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<*mut i32> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *mut *mut i32)
            .cast_const()))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == ((&mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<*mut i32> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *mut *mut i32)
                .cast_const())))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_16() -> bool {
    let mut val: Vec<*const i32> = Vec::from([std::ptr::null(), std::ptr::null()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == (&mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<*const i32> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *mut *const i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == (&mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<*const i32> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *mut *const i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_17() -> bool {
    let mut val: Vec<*const i32> = Vec::from([std::ptr::null(), std::ptr::null()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == (&(*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<*const i32> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *const *const i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == (&(*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<*const i32> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *const *const i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorArray_18() -> bool {
    let mut val: Vec<*const i32> = Vec::from([std::ptr::null(), std::ptr::null()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!())
        == ((&mut (*{
            let __idx = 0_usize as usize;
            let __v: &mut Vec<*const i32> = &mut val;
            __v.as_mut_ptr().add(__idx)
        }) as *mut *const i32)
            .cast_const()))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!())
            == ((&mut (*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<*const i32> = &mut val;
                __v.as_mut_ptr().add(__idx)
            }) as *mut *const i32)
                .cast_const())))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_19() -> bool {
    let val: Vec<i32> = Vec::from([0_i32, 0_i32]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_20() -> bool {
    let val: Vec<i32> = Vec::from([0_i32, 0_i32]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_21() -> bool {
    let val: Vec<i32> = Vec::from([0_i32, 0_i32]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_22() -> bool {
    let val: Vec<i64> = Vec::from([0_i64, 0_i64]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const i64))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const i64)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_23() -> bool {
    let val: Vec<i64> = Vec::from([0_i64, 0_i64]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const i64))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const i64)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_24() -> bool {
    let val: Vec<i64> = Vec::from([0_i64, 0_i64]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const i64))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const i64)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_25() -> bool {
    let val: Vec<f64> = Vec::from([0_f64, 0_f64]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const f64))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const f64)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_26() -> bool {
    let val: Vec<f64> = Vec::from([0_f64, 0_f64]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const f64))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const f64)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_27() -> bool {
    let val: Vec<f64> = Vec::from([0_f64, 0_f64]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const f64))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const f64)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_28() -> bool {
    let val: Vec<A> = Vec::from([<A>::default(), <A>::default()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const A))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const A)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_29() -> bool {
    let val: Vec<A> = Vec::from([<A>::default(), <A>::default()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const A))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const A)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_30() -> bool {
    let val: Vec<A> = Vec::from([<A>::default(), <A>::default()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const A))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const A)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_31() -> bool {
    let val: Vec<*mut i32> = Vec::from([std::ptr::null_mut(), std::ptr::null_mut()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const *mut i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const *mut i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_32() -> bool {
    let val: Vec<*mut i32> = Vec::from([std::ptr::null_mut(), std::ptr::null_mut()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const *mut i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const *mut i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_33() -> bool {
    let val: Vec<*mut i32> = Vec::from([std::ptr::null_mut(), std::ptr::null_mut()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const *mut i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const *mut i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_34() -> bool {
    let val: Vec<*const i32> = Vec::from([std::ptr::null(), std::ptr::null()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const *const i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const *const i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_35() -> bool {
    let val: Vec<*const i32> = Vec::from([std::ptr::null(), std::ptr::null()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const *const i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const *const i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructorConstArray_36() -> bool {
    let val: Vec<*const i32> = Vec::from([std::ptr::null(), std::ptr::null()]);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let mut s1: () = unimplemented!();
    let mut s2: () = unimplemented!();
    return ((((unimplemented!()) == (&(*unimplemented!()) as *const *const i32))
        && ((unimplemented!()) == (2_())))
        && ((unimplemented!()) == (&(*unimplemented!()) as *const *const i32)))
        && ((unimplemented!()) == (2_()));
}
pub unsafe fn testConstructors_37() -> bool {
    const _: () = assert!(true, "testConstructorArray<T, T>()");;
    const _: () = assert!(true, "testConstructorArray<const T, const T>()");;
    const _: () = assert!(true, "testConstructorArray<const T, T>()");;
    const _: () = assert!(true, "testConstructorConstArray<T, T>()");;
    const _: () = assert!(true, "testConstructorConstArray<const T, const T>()");;
    const _: () = assert!(true, "testConstructorConstArray<const T, T>()");;
    return (((((unsafe { testConstructorArray_1() }) && (unsafe { testConstructorArray_2() }))
        && (unsafe { testConstructorArray_3() }))
        && (unsafe { testConstructorConstArray_19() }))
        && (unsafe { testConstructorConstArray_20() }))
        && (unsafe { testConstructorConstArray_21() });
}
pub unsafe fn testConstructors_38() -> bool {
    const _: () = assert!(true, "testConstructorArray<T, T>()");;
    const _: () = assert!(true, "testConstructorArray<const T, const T>()");;
    const _: () = assert!(true, "testConstructorArray<const T, T>()");;
    const _: () = assert!(true, "testConstructorConstArray<T, T>()");;
    const _: () = assert!(true, "testConstructorConstArray<const T, const T>()");;
    const _: () = assert!(true, "testConstructorConstArray<const T, T>()");;
    return (((((unsafe { testConstructorArray_4() }) && (unsafe { testConstructorArray_5() }))
        && (unsafe { testConstructorArray_6() }))
        && (unsafe { testConstructorConstArray_22() }))
        && (unsafe { testConstructorConstArray_23() }))
        && (unsafe { testConstructorConstArray_24() });
}
pub unsafe fn testConstructors_39() -> bool {
    const _: () = assert!(true, "testConstructorArray<T, T>()");;
    const _: () = assert!(true, "testConstructorArray<const T, const T>()");;
    const _: () = assert!(true, "testConstructorArray<const T, T>()");;
    const _: () = assert!(true, "testConstructorConstArray<T, T>()");;
    const _: () = assert!(true, "testConstructorConstArray<const T, const T>()");;
    const _: () = assert!(true, "testConstructorConstArray<const T, T>()");;
    return (((((unsafe { testConstructorArray_7() }) && (unsafe { testConstructorArray_8() }))
        && (unsafe { testConstructorArray_9() }))
        && (unsafe { testConstructorConstArray_25() }))
        && (unsafe { testConstructorConstArray_26() }))
        && (unsafe { testConstructorConstArray_27() });
}
pub unsafe fn testConstructors_40() -> bool {
    const _: () = assert!(true, "testConstructorArray<T, T>()");;
    const _: () = assert!(true, "testConstructorArray<const T, const T>()");;
    const _: () = assert!(true, "testConstructorArray<const T, T>()");;
    const _: () = assert!(true, "testConstructorConstArray<T, T>()");;
    const _: () = assert!(true, "testConstructorConstArray<const T, const T>()");;
    const _: () = assert!(true, "testConstructorConstArray<const T, T>()");;
    return (((((unsafe { testConstructorArray_10() })
        && (unsafe { testConstructorArray_11() }))
        && (unsafe { testConstructorArray_12() }))
        && (unsafe { testConstructorConstArray_28() }))
        && (unsafe { testConstructorConstArray_29() }))
        && (unsafe { testConstructorConstArray_30() });
}
pub unsafe fn testConstructors_41() -> bool {
    const _: () = assert!(true, "testConstructorArray<T, T>()");;
    const _: () = assert!(true, "testConstructorArray<const T, const T>()");;
    const _: () = assert!(true, "testConstructorArray<const T, T>()");;
    const _: () = assert!(true, "testConstructorConstArray<T, T>()");;
    const _: () = assert!(true, "testConstructorConstArray<const T, const T>()");;
    const _: () = assert!(true, "testConstructorConstArray<const T, T>()");;
    return (((((unsafe { testConstructorArray_13() })
        && (unsafe { testConstructorArray_14() }))
        && (unsafe { testConstructorArray_15() }))
        && (unsafe { testConstructorConstArray_31() }))
        && (unsafe { testConstructorConstArray_32() }))
        && (unsafe { testConstructorConstArray_33() });
}
pub unsafe fn testConstructors_42() -> bool {
    const _: () = assert!(true, "testConstructorArray<T, T>()");;
    const _: () = assert!(true, "testConstructorArray<const T, const T>()");;
    const _: () = assert!(true, "testConstructorArray<const T, T>()");;
    const _: () = assert!(true, "testConstructorConstArray<T, T>()");;
    const _: () = assert!(true, "testConstructorConstArray<const T, const T>()");;
    const _: () = assert!(true, "testConstructorConstArray<const T, T>()");;
    return (((((unsafe { testConstructorArray_16() })
        && (unsafe { testConstructorArray_17() }))
        && (unsafe { testConstructorArray_18() }))
        && (unsafe { testConstructorConstArray_34() }))
        && (unsafe { testConstructorConstArray_35() }))
        && (unsafe { testConstructorConstArray_36() });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct A {}
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
    assert!((unsafe { testConstructors_37() }));
    assert!((unsafe { testConstructors_38() }));
    assert!((unsafe { testConstructors_39() }));
    assert!((unsafe { testConstructors_40() }));
    assert!((unsafe { testConstructors_41() }));
    assert!((unsafe { testConstructors_42() }));
    (unsafe { checkCV_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
