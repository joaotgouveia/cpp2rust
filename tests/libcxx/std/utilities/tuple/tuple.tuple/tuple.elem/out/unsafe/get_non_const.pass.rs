extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Empty {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct S {
    pub a: (),
    pub k: i32,
    pub e: Empty,
}
impl S {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            a: unimplemented!(),
            k: ((*unimplemented!()) as i32),
            e: (*unimplemented!()),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut S) -> Self {
        let mut this = Self {
            a: unimplemented!(),
            k: (*_a0).k,
            e: (*_a0).e,
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        unimplemented!();
        self.k = (*_a0).k;
        self.e = (*_a0).e;
        return &mut (*(self as *mut S));
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new() }
    }
}
pub unsafe fn getP_0() -> () {
    return unimplemented!();
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
        let mut t: () = unimplemented!();
        assert!(((*unimplemented!()) == (3)));
        (*unimplemented!()) = (2 as ());
        assert!(((*unimplemented!()) == (2)));
    }
    {
        let mut t: () = unimplemented!();
        assert!(
            (*unimplemented!())
                == std::slice::from_raw_parts(
                    c"high".as_ptr(),
                    libcc2rs::c_str_len(c"high".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((*unimplemented!()) == (5)));
        unimplemented!();
        (*unimplemented!()) = (4 as ());
        assert!(
            (*unimplemented!())
                == std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((*unimplemented!()) == (4)));
    }
    {
        let mut d: f64 = 1.5_f64;
        let mut t: () = unimplemented!();
        assert!(((*unimplemented!()) == (1.5_f64)));
        assert!(
            (*unimplemented!())
                == std::slice::from_raw_parts(
                    c"high".as_ptr(),
                    libcc2rs::c_str_len(c"high".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((*unimplemented!()) == (5)));
        (*unimplemented!()) = 2.5_f64;
        unimplemented!();
        (*unimplemented!()) = (4 as ());
        assert!(((*unimplemented!()) == (2.5_f64)));
        assert!(
            (*unimplemented!())
                == std::slice::from_raw_parts(
                    c"four".as_ptr(),
                    libcc2rs::c_str_len(c"four".as_ptr()) + 1
                )
                .to_vec()
        );
        assert!(((*unimplemented!()) == (4)));
        assert!(((d) == (2.5_f64)));
    }
    {
        const _: () = assert!(
            true,
            "std::get<0> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 0"
        );;
        const _: () = assert!(
            true,
            "std::get<1> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 1"
        );;
        const _: () = assert!(
            true,
            "std::get<2> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 2"
        );;
        const _: () = assert!(
            true,
            "std::get<3> ( std::make_tuple ( 0.0f, 1, 2.0, 3L )) == 3"
        );;
        const _: () = assert!(true, "S().k == 1");;
        const _: () = assert!(true, "std::get<1>(getP()) == 4");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
