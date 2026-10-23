extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
impl sized_allocator_bool__unsigned_char__signed_char_ {
    pub unsafe fn new_1(
        a: *const sized_allocator_unsigned_char__unsigned_char__signed_char_,
    ) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_char__signed_char_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_char__signed_char_ {
    pub unsafe fn new_2(mut d: ()) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_char__signed_char_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    data_: (),
}
impl sized_allocator_unsigned_char__unsigned_char__signed_char_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        if ((n)
            > (unsafe {
                sized_allocator_unsigned_char__unsigned_char__signed_char_::max_size(self)
            }))
        {
            unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl sized_allocator_bool__unsigned_short__short_ {
    pub unsafe fn new_1(a: *const sized_allocator_unsigned_short__unsigned_short__short_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_short__short_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_short__short_ {
    pub unsafe fn new_2(mut d: ()) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_short__unsigned_short__short_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_short__short_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_unsigned_short__unsigned_short__short_ {
    data_: (),
}
impl sized_allocator_unsigned_short__unsigned_short__short_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        if ((n)
            > (unsafe { sized_allocator_unsigned_short__unsigned_short__short_::max_size(self) }))
        {
            unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl sized_allocator_bool__unsigned_int__int_ {
    pub unsafe fn new_1(a: *const sized_allocator_unsigned_int__unsigned_int__int_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_int__int_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_int__int_ {
    pub unsafe fn new_2(mut d: ()) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_int__unsigned_int__int_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_int__int_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_unsigned_int__unsigned_int__int_ {
    data_: (),
}
impl sized_allocator_unsigned_int__unsigned_int__int_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        if ((n) > (unsafe { sized_allocator_unsigned_int__unsigned_int__int_::max_size(self) })) {
            unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
impl sized_allocator_bool__unsigned_long__long_ {
    pub unsafe fn new_1(a: *const sized_allocator_unsigned_long__unsigned_long__long_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_long__long_ {
    data_: (),
}
impl sized_allocator_bool__unsigned_long__long_ {
    pub unsafe fn new_2(mut d: ()) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_long__long_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_unsigned_long__unsigned_long__long_ {
    data_: (),
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        if ((n) > (unsafe { sized_allocator_unsigned_long__unsigned_long__long_::max_size(self) }))
        {
            unimplemented!();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> () {
        return (unimplemented!() as ()).wrapping_div((::std::mem::size_of::<()>() as ()));
    }
}
pub unsafe fn tests_0() -> () {
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c: () = unimplemented!();
        unimplemented!();
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
    (unsafe { tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
