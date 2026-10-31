extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test0_0(_a0: *const ()) {
    const _: () = assert!(true, "std::tuple_size<Tuple>::value == 0");;
}
pub unsafe fn test1a_1(t: *const ()) {
    const _: () = assert!(true, "std::tuple_size<Tuple>::value == 1");;
    const _: () = assert!(
        true,
        "std::is_same<typename std::tuple_element<0, Tuple>::type, int&&>::value"
    );;
    assert!(((*unimplemented!()) == (1)));
}
pub unsafe fn test1b_2(t: *const ()) {
    const _: () = assert!(true, "std::tuple_size<Tuple>::value == 1");;
    const _: () = assert!(
        true,
        "std::is_same<typename std::tuple_element<0, Tuple>::type, int&>::value"
    );;
    assert!(((*unimplemented!()) == (2)));
}
pub unsafe fn test2a_3(t: *const ()) {
    const _: () = assert!(true, "std::tuple_size<Tuple>::value == 2");;
    const _: () = assert!(
        true,
        "std::is_same<typename std::tuple_element<0, Tuple>::type, double&>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<typename std::tuple_element<1, Tuple>::type, char&>::value"
    );;
    assert!(((*unimplemented!()) == (2.5_f64)));
    assert!((((*unimplemented!()) as i32) == (('a' as libc::c_char) as i32)));
}
pub unsafe fn test3_4(_a0: *const ()) -> i32 {
    return (unimplemented!() as i32);
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
    {
        (unsafe {
            let mut _arg0: () = unimplemented!();
            test0_0(&mut _arg0)
        });
    }
    {
        (unsafe {
            let mut _t: () = unimplemented!();
            test1a_1(&mut _t)
        });
    }
    {
        let mut i: i32 = 2;
        (unsafe {
            let mut _t: () = unimplemented!();
            test1b_2(&mut _t)
        });
    }
    {
        let mut i: f64 = 2.5_f64;
        let mut c: libc::c_char = ('a' as libc::c_char);
        (unsafe {
            let mut _t: () = unimplemented!();
            test2a_3(&mut _t)
        });
        const _: () = assert!(true, "test3 (std::forward_as_tuple(i, c)) == 2");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
