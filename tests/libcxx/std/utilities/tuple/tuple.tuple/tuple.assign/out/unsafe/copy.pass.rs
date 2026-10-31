extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct NonAssignable {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct CopyAssignable {}
const _: () = assert!(true, "std::is_copy_assignable<CopyAssignable>::value");
#[repr(C)]
#[derive(FnPtrArg, MoveAssignUnsafe, Default)]
pub struct MoveAssignable {}
impl MoveAssignable {
    pub unsafe fn move_assign(&mut self, _a0: *mut MoveAssignable) -> *mut MoveAssignable {
        return &mut (*(self as *mut MoveAssignable));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct PotentiallyThrowingCopyAssignable {}
impl PotentiallyThrowingCopyAssignable {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const PotentiallyThrowingCopyAssignable,
    ) -> *mut PotentiallyThrowingCopyAssignable {
        return &mut (*(self as *mut PotentiallyThrowingCopyAssignable));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CopyAssignableInt {}
impl CopyAssignableInt {
    pub unsafe fn operator_assign_1(&mut self, _a0: *mut i32) -> *mut CopyAssignableInt {
        return &mut (*(self as *mut CopyAssignableInt));
    }
}
pub unsafe fn test_0() -> bool {
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
    }
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
        assert!(((*unimplemented!()) == (2)));
    }
    {
        let mut t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
        assert!(((*unimplemented!()) == (2)));
        assert!((((*unimplemented!()) as i32) == (('a' as libc::c_char) as i32)));
    }
    {
        let mut x: i32 = 42;
        let mut y: i32 = 100;
        let mut x2: i32 = -1_i32;
        let mut y2: i32 = 500;
        let mut t: () = unimplemented!();
        let mut t2: () = unimplemented!();
        unimplemented!();
        assert!(((*unimplemented!()) == (x2)));
        assert!(((unimplemented!()) == (&mut x as *mut i32)));
        assert!(((*unimplemented!()) == (y2)));
        assert!(((unimplemented!()) == (&mut y as *mut i32)));
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
    {
        let t0: () = unimplemented!();
        let mut t: () = unimplemented!();
        unimplemented!();
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
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P const>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_assignable<T&, P&>::value");;
    }
    {
        const _: () = assert!(true, "std::is_nothrow_copy_assignable<T>::value");;
    }
    {
        const _: () = assert!(true, "std::is_copy_assignable<T>::value");;
        const _: () = assert!(true, "!std::is_nothrow_copy_assignable<T>::value");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
