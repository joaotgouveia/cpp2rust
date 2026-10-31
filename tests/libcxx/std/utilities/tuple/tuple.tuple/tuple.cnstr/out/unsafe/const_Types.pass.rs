extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct NoValueCtor {
    pub id: i32,
}
impl NoValueCtor {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            id: (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc(),
        };
        this
    }
    pub unsafe fn copy_from(other: *const NoValueCtor) -> Self {
        let mut this = Self { id: (*other).id };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const NoValueCtor) -> *mut NoValueCtor {
        self.id = (*_a0).id;
        return &mut (*(self as *mut NoValueCtor));
    }
}
impl Clone for NoValueCtor {
    fn clone(&self) -> Self {
        unsafe { NoValueCtor::copy_from(self as *const NoValueCtor) }
    }
}
impl Default for NoValueCtor {
    fn default() -> Self {
        unsafe { NoValueCtor::new() }
    }
}
pub static mut count_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe)]
pub struct NoValueCtorEmpty {}
impl NoValueCtorEmpty {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_from(_a0: *const NoValueCtorEmpty) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const NoValueCtorEmpty) -> *mut NoValueCtorEmpty {
        return &mut (*(self as *mut NoValueCtorEmpty));
    }
}
impl Clone for NoValueCtorEmpty {
    fn clone(&self) -> Self {
        unsafe { NoValueCtorEmpty::copy_from(self as *const NoValueCtorEmpty) }
    }
}
impl Default for NoValueCtorEmpty {
    fn default() -> Self {
        unsafe { NoValueCtorEmpty::new() }
    }
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, Default)]
pub struct ImplicitCopy {}
impl ImplicitCopy {
    pub unsafe fn new(mut _a0: i32) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_from(_a0: *const ImplicitCopy) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const ImplicitCopy) -> *mut ImplicitCopy {
        return &mut (*(self as *mut ImplicitCopy));
    }
}
impl Clone for ImplicitCopy {
    fn clone(&self) -> Self {
        unsafe { ImplicitCopy::copy_from(self as *const ImplicitCopy) }
    }
}
pub unsafe fn testImplicitCopy1_1() -> () {
    let mut i: ImplicitCopy = ImplicitCopy::new({ 42 });
    return unimplemented!();
}
pub unsafe fn testImplicitCopy2_2() -> () {
    let i: ImplicitCopy = ImplicitCopy::new({ 42 });
    return unimplemented!();
}
pub unsafe fn testImplicitCopy3_3() -> () {
    let i: ImplicitCopy = ImplicitCopy::new({ 42 });
    return i;
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
        let mut t: () = unimplemented!();
        assert!((*unimplemented!()).is_null());
    }
    {
        let mut t: () = unimplemented!();
        assert!(((*unimplemented!()) == (2)));
    }
    {
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::get<0>(t) == 2");;
    }
    {
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::get<0>(t) == 0");;
    }
    {
        let mut t: () = unimplemented!();
        assert!(((*unimplemented!()) == (2)));
        assert!((*unimplemented!()).is_null());
    }
    {
        let t: () = unimplemented!();
        const _: () = assert!(true, "std::get<0>(t) == 2");;
        const _: () = assert!(true, "std::get<1>(t) == nullptr");;
    }
    {
        let mut t: () = unimplemented!();
        assert!(((*unimplemented!()) == (2)));
        assert!((*unimplemented!()).is_null());
    }
    {
        let mut t: () = unimplemented!();
        assert!(((*unimplemented!()) == (2)));
        assert!((*unimplemented!()).is_null());
        assert!(
            (*unimplemented!())
                == std::slice::from_raw_parts(
                    c"text".as_ptr(),
                    libcc2rs::c_str_len(c"text".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut t: () = unimplemented!();
        assert!(((*unimplemented!()) == (1)));
        assert!((((*unimplemented!()).id) == (1)));
        assert!(((*unimplemented!()) == (2)));
        assert!(((*unimplemented!()) == (3)));
    }
    {
        let mut t: () = unimplemented!();
        assert!(((*unimplemented!()) == (1)));
        assert!(((*unimplemented!()) == (2)));
        assert!(((*unimplemented!()) == (3)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
}
