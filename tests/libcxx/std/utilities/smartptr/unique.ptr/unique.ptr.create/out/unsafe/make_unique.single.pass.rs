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
        let mut p1: Option<Box<i32>> = Some(Box::new(1));
        assert!(
            ((*p1
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (1))
        );
        p1 = Some(Box::new(0_i32)).take();
        assert!(
            ((*p1
                .as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const i32 as *mut i32))
                == (0))
        );
    }
    {
        let mut p2: Option<Box<Vec<libc::c_char>>> = Some(Box::new(
            std::slice::from_raw_parts(
                c"Meow!".as_ptr(),
                libcc2rs::c_str_len(c"Meow!".as_ptr()) + 1,
            )
            .to_vec(),
        ));
        assert!(
            (*p2.as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const Vec<libc::c_char>
                    as *mut Vec<libc::c_char>))
                == std::slice::from_raw_parts(
                    c"Meow!".as_ptr(),
                    libcc2rs::c_str_len(c"Meow!".as_ptr()) + 1
                )
                .to_vec()
        );
        p2 = Some(Box::new(vec![libc::c_char::default()])).take();
        assert!(
            (*p2.as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const Vec<libc::c_char>
                    as *mut Vec<libc::c_char>))
                == std::slice::from_raw_parts(c"".as_ptr(), libcc2rs::c_str_len(c"".as_ptr()) + 1)
                    .to_vec()
        );
        p2 = Some(Box::new(
            vec![('z' as libc::c_char); (6 as usize) as usize]
                .iter()
                .cloned()
                .chain(std::iter::once((0 as libc::c_char)))
                .collect(),
        ))
        .take();
        assert!(
            (*p2.as_deref()
                .map_or(::std::ptr::null_mut(), |v| v as *const Vec<libc::c_char>
                    as *mut Vec<libc::c_char>))
                == std::slice::from_raw_parts(
                    c"zzzzzz".as_ptr(),
                    libcc2rs::c_str_len(c"zzzzzz".as_ptr()) + 1
                )
                .to_vec()
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
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
