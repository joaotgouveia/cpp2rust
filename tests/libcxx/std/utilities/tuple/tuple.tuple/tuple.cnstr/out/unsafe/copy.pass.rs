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
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        (&(t));
    }
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        assert!(((*unimplemented!()) == (2)));
    }
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        assert!(((*unimplemented!()) == (2)));
        assert!((((*unimplemented!()) as i32) == (('a' as libc::c_char) as i32)));
    }
    {
        let t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        assert!(((*unimplemented!()) == (2)));
        assert!((((*unimplemented!()) as i32) == (('a' as libc::c_char) as i32)));
        assert!(
            (*unimplemented!())
                == std::slice::from_raw_parts(
                    c"some text".as_ptr(),
                    libcc2rs::c_str_len(c"some text".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let t0: () = unimplemented!();
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::get<0>(t) == 2");;
    }
    {
        let t0: () = unimplemented!();
        let t: () = unimplemented!();
        let e: Empty = (*unimplemented!());
        (&(e));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
