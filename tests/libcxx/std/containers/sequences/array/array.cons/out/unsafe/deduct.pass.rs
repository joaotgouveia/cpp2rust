extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn tests_0() -> bool {
    {
        let mut arr: Vec<i32> = Vec::from([1, 2, 3]);
        const _: () = assert!(true, "std::is_same_v<decltype(arr), std::array<int, 3>>");;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i32> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (1))
        );
        assert!(
            ((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<i32> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (2))
        );
        assert!(
            ((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<i32> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (3))
        );
    }
    {
        let l1: i64 = 42_i64;
        let mut arr: Vec<i64> = Vec::from([1_i64, 4_i64, 9_i64, l1]);
        const _: () = assert!(true, "std::is_same_v<decltype(arr)::value_type, long>");;
        const _: () = assert!(true, "arr.size() == 4");;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (1_i64))
        );
        assert!(
            ((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<i64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (4_i64))
        );
        assert!(
            ((*{
                let __idx = 2_usize as usize;
                let __v: &mut Vec<i64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (9_i64))
        );
        assert!(
            ((*{
                let __idx = 3_usize as usize;
                let __v: &mut Vec<i64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (l1))
        );
    }
    {
        let mut source: Vec<f64> = Vec::from([4_f64, 5_f64]);
        let mut arr: Vec<f64> = source.clone();
        const _: () = assert!(true, "std::is_same_v<decltype(arr), decltype(source)>");;
        const _: () = assert!(true, "std::is_same_v<decltype(arr), std::array<double, 2>>");;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<f64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (4_f64))
        );
        assert!(
            ((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<f64> = &mut arr;
                __v.as_mut_ptr().add(__idx)
            }) == (5_f64))
        );
    }
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
