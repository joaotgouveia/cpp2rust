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
    pub value: (),
    pub value2: (),
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct X {}
impl X {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtor_called_0)) = true;
    }
}
pub static mut dtor_called_0: std::cell::LazyCell<()> =
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
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
            unimplemented!();
        }
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const dtor_called_0);
}
