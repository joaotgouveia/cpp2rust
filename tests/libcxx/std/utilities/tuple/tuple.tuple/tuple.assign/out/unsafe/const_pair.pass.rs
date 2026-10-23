extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, Default)]
pub struct NothrowCopyAssignable {}
impl NothrowCopyAssignable {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const NothrowCopyAssignable,
    ) -> *mut NothrowCopyAssignable {
        return &mut (*(self as *mut NothrowCopyAssignable));
    }
}
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, Default)]
pub struct PotentiallyThrowingCopyAssignable {}
impl PotentiallyThrowingCopyAssignable {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const PotentiallyThrowingCopyAssignable,
    ) -> *mut PotentiallyThrowingCopyAssignable {
        return &mut (*(self as *mut PotentiallyThrowingCopyAssignable));
    }
}
pub unsafe fn test_0() -> () {
    {
        let mut t0: () = {
            let mut __tmp_0: () = 2_();
            let mut __tmp_1: () = ('a' as ());
            std_pair_long__char_::new({ &mut __tmp_0 }, { &mut __tmp_1 })
        };
        let mut t1: () = unimplemented!();
        t1 = (t0).clone();
        unimplemented!();
        unimplemented!();
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    {
        const _: () = assert!(true, "!std::is_assignable<T&, const P &>::value");;
    }
    {
        const _: () = assert!(
            true,
            "std::is_nothrow_assignable<Tuple&, Pair const&>::value"
        );;
        const _: () = assert!(true, "std::is_nothrow_assignable<Tuple&, Pair&>::value");;
        const _: () = assert!(
            true,
            "std::is_nothrow_assignable<Tuple&, Pair const&&>::value"
        );;
    }
    {
        const _: () = assert!(true, "std::is_assignable<Tuple&, Pair const&>::value");;
        const _: () = assert!(
            true,
            "!std::is_nothrow_assignable<Tuple&, Pair const&>::value"
        );;
        const _: () = assert!(true, "std::is_assignable<Tuple&, Pair&>::value");;
        const _: () = assert!(true, "!std::is_nothrow_assignable<Tuple&, Pair&>::value");;
        const _: () = assert!(true, "std::is_assignable<Tuple&, Pair const&&>::value");;
        const _: () = assert!(
            true,
            "!std::is_nothrow_assignable<Tuple&, Pair const&&>::value"
        );;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
