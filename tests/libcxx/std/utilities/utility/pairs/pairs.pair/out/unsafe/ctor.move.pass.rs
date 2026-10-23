extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, Default)]
pub struct Dummy {}
impl Dummy {
    pub unsafe fn move_from(_a0: *mut Dummy) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NotCopyOrMoveConstructible {}
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
        const _: () = assert!(true, "std::is_move_constructible<P1>::value");;
        let mut p1: () = {
            let mut __tmp_0: () = 3_();
            let mut __tmp_1: () = 4_();
            std_pair_int__short_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
        };
        let mut p2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        const _: () = assert!(true, "!std::is_copy_constructible<P>::value");;
        const _: () = assert!(true, "std::is_move_constructible<P>::value");;
    }
    {
        {
            const _: () = assert!(true, "std::is_move_constructible<P>::value");;
            let mut obj: NotCopyOrMoveConstructible = <NotCopyOrMoveConstructible>::default();
            let mut p2: () = {
                let mut __tmp_2: () = 3_();
                std_pair_NotCopyOrMoveConstructible_ref__int_::new({ &mut obj }, { &mut __tmp_2 })
            };
            let mut p1: () = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            const _: () = assert!(true, "std::is_move_constructible<P>::value");;
            let mut obj: NotCopyOrMoveConstructible = <NotCopyOrMoveConstructible>::default();
            let mut p2: () = {
                let mut __tmp_3: () = 3_();
                std_pair_NotCopyOrMoveConstructible_refref__int_::new({ &mut obj }, {
                    &mut __tmp_3
                })
            };
            let mut p1: () = unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
