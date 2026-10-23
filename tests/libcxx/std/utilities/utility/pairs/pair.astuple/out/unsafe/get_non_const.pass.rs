extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg)]
pub struct S {
    pub a: (),
    pub k: (),
}
impl S {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            a: {
                let mut __tmp_0: () = 1_();
                let mut __tmp_1: () = 2_();
                std_pair_int__int_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
            },
            k: (*(unsafe { get_0(&mut this.a) })),
        };
        this
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new() }
    }
}
pub unsafe fn getP_1() -> () {
    return {
        let mut __tmp_2: () = 3_();
        let mut __tmp_3: () = 4_();
        std_pair_int__int_::new({ &mut __tmp_2 }, { &mut __tmp_3 })
    };
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    {
        let mut p: () = {
            let mut __tmp_4: () = 3_();
            let mut __tmp_5: () = 4_();
            std_pair_int__short_::new({ &mut __tmp_4 }, { &mut __tmp_5 })
        };
        unimplemented!();
        unimplemented!();
        (*(unsafe { get_2(&mut p) })) = 5_();
        (*(unsafe { get_3(&mut p) })) = 6_();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "S().k == 1");;
        const _: () = assert!(true, "std::get<1>(getP()) == 4");;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
