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
pub struct PODType {
    pub value: i32,
    pub value2: i32,
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct X {}
impl X {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtor_called_0)) = true;
    }
}
pub static mut dtor_called_0: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });

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
        const _: () = assert!(true, "std::is_trivially_destructible<T>::value");;
        const _: () = assert!(true, "std::is_trivially_destructible<optional<T>>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_destructible<T>::value");;
        const _: () = assert!(true, "std::is_trivially_destructible<optional<T>>::value");;
    }
    {
        const _: () = assert!(true, "std::is_trivially_destructible<T>::value");;
        const _: () = assert!(true, "std::is_trivially_destructible<optional<T>>::value");;
    }
    {
        const _: () = assert!(true, "!std::is_trivially_destructible<T>::value");;
        const _: () = assert!(true, "!std::is_trivially_destructible<optional<T>>::value");;
        {
            let mut x: X = <X>::default();
            let _dtor_x = ScopedDestructorUnsafe::new(&raw mut x, X::destructor);
            let mut opt: () = unimplemented!();
            assert!(
                (((*std::cell::LazyCell::force_mut(&mut *&raw mut dtor_called_0)) as i32)
                    == (false as i32))
            );
        }
        assert!(
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut dtor_called_0)) as i32)
                == (true as i32))
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const dtor_called_0);
}
