extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> bool {
    {
        let mut i: i32 = 1;
        let mut j: i32 = 2;
        {
            let mut p: (*mut i32, *mut i32) = (i, j);
            assert!(((unimplemented!()) == (&mut i as *mut i32)));
            assert!(((unimplemented!()) == (&mut j as *mut i32)));
            assert!(((unimplemented!()) == (&mut i as *mut i32)));
            assert!(((*unimplemented!()) == (2)));
            let cp: (*mut i32, *mut i32) = (i, j);
            assert!(((unimplemented!()) == (&mut i as *mut i32)));
            assert!(((unimplemented!()) == (&mut j as *mut i32)));
            assert!(((unimplemented!()) == (&mut i as *mut i32)));
            assert!(((*unimplemented!()) == (2)));
        }
        {
            let mut p: (*mut i32, *mut i32) = (i, j);
            assert!(((unimplemented!()) == (&mut j as *mut i32)));
            assert!(((unimplemented!()) == (&mut i as *mut i32)));
            assert!(((unimplemented!()) == (&mut j as *mut i32)));
            assert!(((*unimplemented!()) == (1)));
            let cp: (*mut i32, *mut i32) = (i, j);
            assert!(((unimplemented!()) == (&mut j as *mut i32)));
            assert!(((unimplemented!()) == (&mut i as *mut i32)));
            assert!(((unimplemented!()) == (&mut j as *mut i32)));
            assert!(((*unimplemented!()) == (1)));
        }
    }
    {
        let mut t1: (i32, ()) = (42, unimplemented!());
        assert!(((*unimplemented!()) == (42)));
        assert!(((unimplemented!()) == (1_f32)));
        assert!(((unimplemented!()) == (2_f32)));
    }
    {
        let p1: (i32, i32) = (1, 2);
        let i1: *const i32 = unimplemented!();
        let i2: *const i32 = unimplemented!();
        assert!(((*i1) == (1)));
        assert!(((*i2) == (2)));
    }
    {
        let mut x: i32 = 42;
        let y: i32 = 43;
        let p: (*mut i32, *const i32) = unimplemented!();
        const _: () = assert!(
            true,
            "std::is_same<int&, decltype(std::get<int&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&>(std::move(p)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&, decltype(std::get<int const&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&>(std::move(p)))");;
    }
    {
        let mut x: i32 = 42;
        let y: i32 = 43;
        let p: (*mut i32, *const i32) = (x, y);
        const _: () = assert!(
            true,
            "std::is_same<int&&, decltype(std::get<int&&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&&>(std::move(p)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&&, decltype(std::get<int const&&>(std::move(p)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&&>(std::move(p)))");;
    }
    {
        let p: (i32, i32) = (1, 2);
        const _: () = assert!(true, "std::get<int>(std::move(p)) == 1");;
        const _: () = assert!(true, "std::get<const int>(std::move(p)) == 2");;
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    {
        let mut t: (Option<Box<i32>>, i32) = (
            Some(Box::from_raw((Box::leak(Box::new(4)) as *mut i32))),
            42,
        );
        let mut p: Option<Box<i32>> = (*unimplemented!()).take();
        assert!(
            ((*p.as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!((*unimplemented!()).is_none());
    }
    {
        let t: (Option<Box<i32>>, i32) = (
            Some(Box::from_raw((Box::leak(Box::new(4)) as *mut i32))),
            42,
        );
        const _: () = assert!(
            true,
            "std::is_same<const upint&&, decltype(std::get<upint>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<upint>(std::move(t)))");;
        const _: () = assert!(
            true,
            "std::is_same<const int&&, decltype(std::get<int>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int>(std::move(t)))");;
        let p: *const Option<Box<i32>> = unimplemented!();
        let i: *const i32 = unimplemented!();
        assert!(
            ((*(*p)
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(((*i) == (42)));
        assert!(!((*unimplemented!()).is_none()));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
