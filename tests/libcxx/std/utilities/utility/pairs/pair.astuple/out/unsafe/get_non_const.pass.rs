extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, MoveAssignUnsafe)]
pub struct S {
    pub a: (i32, i32),
    pub k: i32,
}
impl S {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            a: (1, 2),
            k: (*<(i32, i32) as PairGet<0>>::pair_get_mut(&mut this.a)),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut S) -> *mut S {
        self.a = std::mem::take(&mut (*_a0).a);
        self.k = (*_a0).k;
        return &mut (*(self as *mut S));
    }
}
impl Default for S {
    fn default() -> Self {
        unsafe { S::new() }
    }
}
pub unsafe fn getP_0() -> (i32, i32) {
    return (3, 4);
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
        let mut p: (i32, i16) = (3, 4_i16);
        assert!(((*<(i32, i16) as PairGet<0>>::pair_get_mut(&mut p)) == (3)));
        assert!((((*<(i32, i16) as PairGet<1>>::pair_get_mut(&mut p)) as i32) == (4)));
        (*<(i32, i16) as PairGet<0>>::pair_get_mut(&mut p)) = 5;
        (*<(i32, i16) as PairGet<1>>::pair_get_mut(&mut p)) = 6_i16;
        assert!(((*<(i32, i16) as PairGet<0>>::pair_get_mut(&mut p)) == (5)));
        assert!((((*<(i32, i16) as PairGet<1>>::pair_get_mut(&mut p)) as i32) == (6)));
    }
    {
        const _: () = assert!(true, "S().k == 1");;
        const _: () = assert!(true, "std::get<1>(getP()) == 4");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
