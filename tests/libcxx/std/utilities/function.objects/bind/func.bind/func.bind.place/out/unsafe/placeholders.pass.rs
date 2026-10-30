extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(t: *const ()) {
    {
        let mut x: () = unimplemented!();
        &(x);
    }
    {
        let mut x: () = (*t).clone();
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
}
pub unsafe fn test_1(t: *const ()) {
    {
        let mut x: () = unimplemented!();
        &(x);
    }
    {
        let mut x: () = (*t).clone();
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
}
pub unsafe fn test_2(t: *const ()) {
    {
        let mut x: () = unimplemented!();
        &(x);
    }
    {
        let mut x: () = (*t).clone();
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
}
pub unsafe fn test_3(t: *const ()) {
    {
        let mut x: () = unimplemented!();
        &(x);
    }
    {
        let mut x: () = (*t).clone();
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
}
pub unsafe fn test_4(t: *const ()) {
    {
        let mut x: () = unimplemented!();
        &(x);
    }
    {
        let mut x: () = (*t).clone();
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
}
pub unsafe fn test_5(t: *const ()) {
    {
        let mut x: () = unimplemented!();
        &(x);
    }
    {
        let mut x: () = (*t).clone();
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
}
pub unsafe fn test_6(t: *const ()) {
    {
        let mut x: () = unimplemented!();
        &(x);
    }
    {
        let mut x: () = (*t).clone();
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
}
pub unsafe fn test_7(t: *const ()) {
    {
        let mut x: () = unimplemented!();
        &(x);
    }
    {
        let mut x: () = (*t).clone();
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
}
pub unsafe fn test_8(t: *const ()) {
    {
        let mut x: () = unimplemented!();
        &(x);
    }
    {
        let mut x: () = (*t).clone();
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
}
pub unsafe fn test_9(t: *const ()) {
    {
        let mut x: () = unimplemented!();
        &(x);
    }
    {
        let mut x: () = (*t).clone();
        &(x);
        const _: () = assert!(true, "std::is_nothrow_copy_constructible<T>::value");;
        const _: () = assert!(true, "std::is_nothrow_move_constructible<T>::value");;
    }
}
pub unsafe fn test_all_10() -> () {
    (unsafe { test_0(&(*unimplemented!())) });
    (unsafe { test_1(&(*unimplemented!())) });
    (unsafe { test_2(&(*unimplemented!())) });
    (unsafe { test_3(&(*unimplemented!())) });
    (unsafe { test_4(&(*unimplemented!())) });
    (unsafe { test_5(&(*unimplemented!())) });
    (unsafe { test_6(&(*unimplemented!())) });
    (unsafe { test_7(&(*unimplemented!())) });
    (unsafe { test_8(&(*unimplemented!())) });
    (unsafe { test_9(&(*unimplemented!())) });
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
