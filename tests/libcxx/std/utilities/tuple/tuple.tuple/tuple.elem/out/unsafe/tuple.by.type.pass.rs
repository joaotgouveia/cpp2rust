extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
        let mut t1: () = unimplemented!();
        assert!(((*unimplemented!()) == (42)));
        assert!(
            (*unimplemented!())
                == std::slice::from_raw_parts(
                    c"Hi".as_ptr(),
                    libcc2rs::c_str_len(c"Hi".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((unimplemented!()) == (1_f32)));
        assert!(((unimplemented!()) == (2_f32)));
    }
    {
        let mut t2: () = unimplemented!();
        assert!(
            (*unimplemented!())
                == std::slice::from_raw_parts(
                    c"Hi".as_ptr(),
                    libcc2rs::c_str_len(c"Hi".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!((unimplemented!()));
    }
    {
        let p5: () = unimplemented!();
        const _: () = assert!(true, "std::get<int>(p5) == 1");;
        const _: () = assert!(true, "std::get<const int>(p5) == 2");;
    }
    {
        let p5: () = unimplemented!();
        let i1: *const i32 = unimplemented!();
        let i2: *const i32 = unimplemented!();
        assert!(((*i1) == (1)));
        assert!(((*i2) == (2)));
    }
    {
        let mut t: () = unimplemented!();
        let mut p: Option<Box<i32>> = (*unimplemented!()).take();
        assert!(
            ((*p.as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!((*unimplemented!()).is_none());
    }
    {
        let t: () = unimplemented!();
        let p: *const Option<Box<i32>> = unimplemented!();
        assert!(
            ((*(*p)
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (4))
        );
        assert!(!((*unimplemented!()).is_none()));
    }
    {
        let mut x: i32 = 42;
        let mut y: i32 = 43;
        let t: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::is_same<int&, decltype(std::get<int&>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&>(std::move(t)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&, decltype(std::get<int const&>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&>(std::move(t)))");;
    }
    {
        let mut x: i32 = 42;
        let mut y: i32 = 43;
        let t: () = unimplemented!();
        const _: () = assert!(
            true,
            "std::is_same<int&&, decltype(std::get<int&&>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int&&>(std::move(t)))");;
        const _: () = assert!(
            true,
            "std::is_same<int const&&, decltype(std::get<int const&&>(std::move(t)))>::value"
        );;
        const _: () = assert!(true, "noexcept(std::get<int const&&>(std::move(t)))");;
    }
    {
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::get<int>(std::move(t)) == 1");;
        const _: () = assert!(true, "std::get<const int>(std::move(t)) == 2");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
