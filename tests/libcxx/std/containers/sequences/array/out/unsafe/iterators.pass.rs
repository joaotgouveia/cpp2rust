extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct NoDefault {}
impl NoDefault {
    pub unsafe fn new(mut _a0: i32) -> Self {
        let mut this = Self {};
        this
    }
}
pub unsafe fn check_noexcept_0(c: *mut Vec<i32>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: *const Vec<i32> = c;
    &(*cc);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn check_noexcept_1(c: *mut Vec<i32>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: *const Vec<i32> = c;
    &(*cc);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn check_noexcept_2(c: *mut Vec<i32>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: *const Vec<i32> = c;
    &(*cc);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn check_noexcept_3(c: *mut Vec<i32>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: *const Vec<i32> = c;
    &(*cc);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn check_noexcept_4(c: *mut Vec<f64>) {
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    let cc: *const Vec<f64> = c;
    &(*cc);
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn tests_5() -> bool {
    {
        let mut array: Vec<i32> = std::array::from_fn::<_, 5, _>(|_| 0_i32).to_vec();
        (unsafe { check_noexcept_0(&mut array) });
        let mut i: *mut i32 = unimplemented!();
        let mut j: *const i32 = unimplemented!();
        assert!((((i).cast_const()) == (j)));
    }
    {
        let mut array: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        (unsafe { check_noexcept_1(&mut array) });
        let mut i: *mut i32 = unimplemented!();
        let mut j: *const i32 = unimplemented!();
        assert!((((i).cast_const()) == (j)));
    }
    {
        let mut array: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
        (unsafe { check_noexcept_1(&mut array) });
        let mut i: *mut i32 = unimplemented!();
        let mut j: *const i32 = unimplemented!();
        assert!(((i) == (unimplemented!())));
        assert!(((j) == (unimplemented!())));
    }
    {
        let mut array: Vec<i32> = Vec::from([1]);
        (unsafe { check_noexcept_2(&mut array) });
        let mut i: *mut i32 = unimplemented!();
        assert!(((*i) == (1)));
        assert!(((&mut (*i) as *mut i32) == (array.as_mut_ptr())));
        (*i) = 99;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i32> = &mut array;
                __v.as_mut_ptr().add(__idx)
            }) == (99))
        );
    }
    {
        let mut array: Vec<i32> = Vec::from([1, 2]);
        (unsafe { check_noexcept_3(&mut array) });
        let mut i: *mut i32 = unimplemented!();
        assert!(((*i) == (1)));
        assert!(((&mut (*i) as *mut i32) == (array.as_mut_ptr())));
        (*i) = 99;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<i32> = &mut array;
                __v.as_mut_ptr().add(__idx)
            }) == (99))
        );
        assert!(
            ((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<i32> = &mut array;
                __v.as_mut_ptr().add(__idx)
            }) == (2))
        );
    }
    {
        let mut array: Vec<f64> = Vec::from([1_f64, 2_f64, 3.5_f64]);
        (unsafe { check_noexcept_4(&mut array) });
        let mut i: *mut f64 = unimplemented!();
        assert!(((*i) == (1_f64)));
        assert!(((&mut (*i) as *mut f64) == (array.as_mut_ptr())));
        (*i) = 5.5_f64;
        assert!(
            ((*{
                let __idx = 0_usize as usize;
                let __v: &mut Vec<f64> = &mut array;
                __v.as_mut_ptr().add(__idx)
            }) == (5.5_f64))
        );
        assert!(
            ((*{
                let __idx = 1_usize as usize;
                let __v: &mut Vec<f64> = &mut array;
                __v.as_mut_ptr().add(__idx)
            }) == (2_f64))
        );
    }
    {
        let mut array: Vec<NoDefault> =
            std::array::from_fn::<_, 0, _>(|_| <NoDefault>::default()).to_vec();
        let mut ib: *mut NoDefault = unimplemented!();
        let mut ie: *mut NoDefault = unimplemented!();
        assert!(((ib) == (ie)));
    }
    {
        {
            let mut ii1: *mut i32 = ::std::ptr::null_mut();
            let mut ii2: *mut i32 = ::std::ptr::null_mut();
            let mut ii4: *mut i32 = ii1;
            let mut cii: *const i32 = ::std::ptr::null();
            assert!(((ii1) == (ii2)));
            assert!(((ii1) == (ii4)));
            assert!((((ii1).cast_const()) == (cii)));
            assert!(!((ii1) != (ii2)));
            assert!(!(((ii1).cast_const()) != (cii)));
            let mut c: Vec<i32> = std::array::from_fn::<_, 5, _>(|_| 0_i32).to_vec();
            (unsafe { check_noexcept_0(&mut c) });
            assert!(((unimplemented!()) == (unimplemented!())));
            assert!(((unimplemented!()) == (unimplemented!())));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(((unimplemented!()) == (unimplemented!())));
            assert!(((unimplemented!()) == (unimplemented!())));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(((unimplemented!()) != (unimplemented!())));
            assert!(unimplemented!() != unimplemented!());
            assert!(((unimplemented!()) != (unimplemented!())));
            assert!(unimplemented!() != unimplemented!());
            let mut r1: std::cmp::Ordering = std::cmp::Ord::cmp(&(ii1), &(ii2));
            assert!(r1 == std::cmp::Ordering::Equal);
            let mut r2: std::cmp::Ordering = std::cmp::Ord::cmp(&(cii), &((ii2).cast_const()));
            assert!(r2 == std::cmp::Ordering::Equal);
        }
        {
            let mut ii1: *mut i32 = ::std::ptr::null_mut();
            let mut ii2: *mut i32 = ::std::ptr::null_mut();
            let mut ii4: *mut i32 = ii1;
            let mut cii: *const i32 = ::std::ptr::null();
            assert!(((ii1) == (ii2)));
            assert!(((ii1) == (ii4)));
            assert!(!((ii1) != (ii2)));
            assert!((((ii1).cast_const()) == (cii)));
            assert!(((cii) == ((ii1).cast_const())));
            assert!(!(((ii1).cast_const()) != (cii)));
            assert!(!((cii) != ((ii1).cast_const())));
            assert!(!(((ii1).cast_const()) < (cii)));
            assert!(!((cii) < ((ii1).cast_const())));
            assert!((((ii1).cast_const()) <= (cii)));
            assert!(((cii) <= ((ii1).cast_const())));
            assert!(!(((ii1).cast_const()) > (cii)));
            assert!(!((cii) > ((ii1).cast_const())));
            assert!((((ii1).cast_const()) >= (cii)));
            assert!(((cii) >= ((ii1).cast_const())));
            assert!(
                ((((cii as usize - ii1 as usize) / ::std::mem::size_of::<i32>()) as i64)
                    == (0_i64))
            );
            assert!(
                ((((ii1 as usize - cii as usize) / ::std::mem::size_of::<i32>()) as i64)
                    == (0_i64))
            );
            let mut c: Vec<i32> = std::array::from_fn::<_, 0, _>(|_| 0_i32).to_vec();
            (unsafe { check_noexcept_1(&mut c) });
            assert!(((unimplemented!()) == (unimplemented!())));
            assert!(((unimplemented!()) == (unimplemented!())));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(((unimplemented!()) == (unimplemented!())));
            assert!(((unimplemented!()) == (unimplemented!())));
            assert!(unimplemented!());
            assert!(unimplemented!());
            assert!(((unimplemented!()) == (unimplemented!())));
            assert!(unimplemented!());
            assert!(((unimplemented!()) == (unimplemented!())));
            assert!(unimplemented!());
            let mut r1: std::cmp::Ordering = std::cmp::Ord::cmp(&(ii1), &(ii2));
            assert!(r1 == std::cmp::Ordering::Equal);
            let mut r2: std::cmp::Ordering = std::cmp::Ord::cmp(&(cii), &((ii2).cast_const()));
            assert!(r2 == std::cmp::Ordering::Equal);
        }
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
    (unsafe { tests_5() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
