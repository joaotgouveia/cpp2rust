extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(t: *const std_placeholders___ph_1_) {
    {
        let mut x: std_placeholders___ph_1_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_1_>() };
        &(x);
    }
    {
        let mut x: std_placeholders___ph_1_ = (*t);
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let mut x: std_placeholders___ph_1_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_1_>() };
        x = (*t);
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub unsafe fn test_1(t: *const std_placeholders___ph_2_) {
    {
        let mut x: std_placeholders___ph_2_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_2_>() };
        &(x);
    }
    {
        let mut x: std_placeholders___ph_2_ = (*t);
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let mut x: std_placeholders___ph_2_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_2_>() };
        x = (*t);
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub unsafe fn test_2(t: *const std_placeholders___ph_3_) {
    {
        let mut x: std_placeholders___ph_3_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_3_>() };
        &(x);
    }
    {
        let mut x: std_placeholders___ph_3_ = (*t);
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let mut x: std_placeholders___ph_3_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_3_>() };
        x = (*t);
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub unsafe fn test_3(t: *const std_placeholders___ph_4_) {
    {
        let mut x: std_placeholders___ph_4_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_4_>() };
        &(x);
    }
    {
        let mut x: std_placeholders___ph_4_ = (*t);
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let mut x: std_placeholders___ph_4_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_4_>() };
        x = (*t);
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub unsafe fn test_4(t: *const std_placeholders___ph_5_) {
    {
        let mut x: std_placeholders___ph_5_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_5_>() };
        &(x);
    }
    {
        let mut x: std_placeholders___ph_5_ = (*t);
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let mut x: std_placeholders___ph_5_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_5_>() };
        x = (*t);
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub unsafe fn test_5(t: *const std_placeholders___ph_6_) {
    {
        let mut x: std_placeholders___ph_6_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_6_>() };
        &(x);
    }
    {
        let mut x: std_placeholders___ph_6_ = (*t);
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let mut x: std_placeholders___ph_6_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_6_>() };
        x = (*t);
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub unsafe fn test_6(t: *const std_placeholders___ph_7_) {
    {
        let mut x: std_placeholders___ph_7_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_7_>() };
        &(x);
    }
    {
        let mut x: std_placeholders___ph_7_ = (*t);
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let mut x: std_placeholders___ph_7_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_7_>() };
        x = (*t);
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub unsafe fn test_7(t: *const std_placeholders___ph_8_) {
    {
        let mut x: std_placeholders___ph_8_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_8_>() };
        &(x);
    }
    {
        let mut x: std_placeholders___ph_8_ = (*t);
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let mut x: std_placeholders___ph_8_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_8_>() };
        x = (*t);
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub unsafe fn test_8(t: *const std_placeholders___ph_9_) {
    {
        let mut x: std_placeholders___ph_9_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_9_>() };
        &(x);
    }
    {
        let mut x: std_placeholders___ph_9_ = (*t);
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let mut x: std_placeholders___ph_9_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_9_>() };
        x = (*t);
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub unsafe fn test_9(t: *const std_placeholders___ph_10_) {
    {
        let mut x: std_placeholders___ph_10_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_10_>() };
        &(x);
    }
    {
        let mut x: std_placeholders___ph_10_ = (*t);
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
    {
        let mut x: std_placeholders___ph_10_ =
            unsafe { std::mem::zeroed::<std_placeholders___ph_10_>() };
        x = (*t);
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_assignable<T>::value");;
    }
}
pub unsafe fn test_all_10() -> () {
    (unsafe { test_0(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _1_11))) });
    (unsafe { test_1(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _2_12))) });
    (unsafe { test_2(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _3_13))) });
    (unsafe { test_3(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _4_14))) });
    (unsafe { test_4(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _5_15))) });
    (unsafe { test_5(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _6_16))) });
    (unsafe { test_6(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _7_17))) });
    (unsafe { test_7(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _8_18))) });
    (unsafe { test_8(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _9_19))) });
    (unsafe { test_9(&(*std::cell::LazyCell::force_mut(&mut *&raw mut _10_20))) });
    return true;
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
    (unsafe { test_all_10() });
    const _: () = assert!(true, "test_all()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
