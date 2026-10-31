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
        let mut p1: (i32, i16) = (3, 4_i16);
        assert!(((p1.0) == (3)));
        assert!(((p1.1 as i32) == (4)));
    }
    {
        let mut p1: (Option<Box<i32>>, i16) = (
            Some(Box::from_raw((Box::leak(Box::new(3)) as *mut i32))),
            4_i16,
        );
        assert!(
            ((*p1
                .0
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (3))
        );
        assert!(((p1.1 as i32) == (4)));
    }
    {
        let mut p1: (Option<Box<i32>>, i16) = {
            let mut __a0 = (Default::default(), 4_i16);
            (
                <Option<Box<i32>> as CppFromUnsafe<()>>::cpp_from_move(&mut __a0.0),
                (*&mut __a0.1),
            )
        };
        assert!(p1.0.is_none());
        assert!(((p1.1 as i32) == (4)));
    }
    {
        let p1: (i32, i16) = (3, 4_i16);
        const _: () = assert!(true, "p1.first == 3");;
        const _: () = assert!(true, "p1.second == 4");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
