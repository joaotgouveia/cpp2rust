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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        const _: () = assert!(true, "std::is_move_constructible<P1>::value");;
        let mut p1: (i32, i16) = (3, 4_i16);
        let mut p2: (i32, i16) = std::mem::take(&mut p1);
        assert!(((p2.0) == (3)));
        assert!(((p2.1 as i32) == (4)));
    }
    {
        const _: () = assert!(true, "!std::is_copy_constructible<P>::value");;
        const _: () = assert!(true, "std::is_move_constructible<P>::value");;
    }
    {
        {
            const _: () = assert!(true, "std::is_move_constructible<P>::value");;
            let mut obj: NotCopyOrMoveConstructible = <NotCopyOrMoveConstructible>::default();
            let mut p2: (*mut NotCopyOrMoveConstructible, i32) = (obj, 3);
            let mut p1: (*mut NotCopyOrMoveConstructible, i32) = std::mem::take(&mut p2);
            assert!(((p1.0) == (&mut obj as *mut NotCopyOrMoveConstructible)));
            assert!(((p2.0) == (&mut obj as *mut NotCopyOrMoveConstructible)));
        }
        {
            const _: () = assert!(true, "std::is_move_constructible<P>::value");;
            let mut obj: NotCopyOrMoveConstructible = <NotCopyOrMoveConstructible>::default();
            let mut p2: (*mut NotCopyOrMoveConstructible, i32) = (obj, 3);
            let mut p1: (*mut NotCopyOrMoveConstructible, i32) = std::mem::take(&mut p2);
            assert!(((p1.0) == (&mut obj as *mut NotCopyOrMoveConstructible)));
            assert!(((p2.0) == (&mut obj as *mut NotCopyOrMoveConstructible)));
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
