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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        (&(t));
    }
    {
        let mut t0: () = 2_();
        let mut t: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut t0: () = {
            let mut __tmp_0: () = 2_();
            let mut __tmp_1: () = ('a' as ());
            std_tuple_int__char_::new_1({ &mut __tmp_0 }, { &mut __tmp_1 })
        };
        let mut t: () = std_tuple_int__char_::new_2({ &mut t0 });
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: () = {
            let mut __tmp_2: () = 2_();
            let mut __tmp_3: () = ('a' as ());
            std_tuple_int__char__std_string_::new({ &mut __tmp_2 }, { &mut __tmp_3 }, {
                &std::mem::transmute(*b"some text\0")
            })
        };
        let mut t: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let t0: () = 2_();
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::get<0>(t) == 2");;
    }
    {
        let t0: () = unimplemented!();
        let t: () = unimplemented!();
        let e: Empty = (*(unsafe { get_0(&t) }));
        (&(e));
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
