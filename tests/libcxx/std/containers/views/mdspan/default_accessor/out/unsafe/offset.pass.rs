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
pub struct MinimalElementType {
    pub val: (),
}
impl MinimalElementType {
    pub unsafe fn new_1(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct ElementPool_int__10_ {
    ptr_: *mut (),
}
impl ElementPool_int__10_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this.ptr_ = (unimplemented!() as *mut ()).cast::<()>();
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            unimplemented!();
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn get_ptr(&mut self) -> *mut () {
        return (self.ptr_ as *mut ()).cast::<()>();
    }
    pub unsafe fn destructor(&mut self) {
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
impl Default for ElementPool_int__10_ {
    fn default() -> Self {
        unsafe { ElementPool_int__10_::new() }
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct ElementPool_MinimalElementType__10_ {
    ptr_: *mut MinimalElementType,
}
impl ElementPool_MinimalElementType__10_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this.ptr_ = unimplemented!();
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            unimplemented!();
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn get_ptr(&mut self) -> *mut MinimalElementType {
        return self.ptr_;
    }
    pub unsafe fn destructor(&mut self) {
        let mut i: () = 0_();
        'loop_: while ((i) != (10_())) {
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
}
impl Default for ElementPool_MinimalElementType__10_ {
    fn default() -> Self {
        unsafe { ElementPool_MinimalElementType__10_::new() }
    }
}
pub unsafe fn test_offset_0() {
    let mut data: ElementPool_int__10_ = ElementPool_int__10_::new();
    let _dtor_data = ScopedDestructorUnsafe::new(&raw mut data, ElementPool_int__10_::destructor);
    let mut ptr: *mut () = (unsafe { ElementPool_int__10_::get_ptr(&mut data) });
    let mut acc: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(acc.offset(ptr, i)), typename std::default_accessor<T>::data_handle_type>"
        );;
        const _: () = assert!(true, "");;
        unimplemented!();
        i.postfix_inc();
    }
}
pub unsafe fn test_offset_1() {
    let mut data: ElementPool_int__10_ = ElementPool_int__10_::new();
    let _dtor_data = ScopedDestructorUnsafe::new(&raw mut data, ElementPool_int__10_::destructor);
    let mut ptr: *const () = (unsafe { ElementPool_int__10_::get_ptr(&mut data) }).cast_const();
    let mut acc: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(acc.offset(ptr, i)), typename std::default_accessor<T>::data_handle_type>"
        );;
        const _: () = assert!(true, "");;
        unimplemented!();
        i.postfix_inc();
    }
}
pub unsafe fn test_offset_2() {
    let mut data: ElementPool_MinimalElementType__10_ = ElementPool_MinimalElementType__10_::new();
    let _dtor_data = ScopedDestructorUnsafe::new(
        &raw mut data,
        ElementPool_MinimalElementType__10_::destructor,
    );
    let mut ptr: *mut MinimalElementType =
        (unsafe { ElementPool_MinimalElementType__10_::get_ptr(&mut data) });
    let mut acc: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(acc.offset(ptr, i)), typename std::default_accessor<T>::data_handle_type>"
        );;
        const _: () = assert!(true, "");;
        unimplemented!();
        i.postfix_inc();
    }
}
pub unsafe fn test_offset_3() {
    let mut data: ElementPool_MinimalElementType__10_ = ElementPool_MinimalElementType__10_::new();
    let _dtor_data = ScopedDestructorUnsafe::new(
        &raw mut data,
        ElementPool_MinimalElementType__10_::destructor,
    );
    let mut ptr: *const MinimalElementType =
        (unsafe { ElementPool_MinimalElementType__10_::get_ptr(&mut data) }).cast_const();
    let mut acc: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (10_())) {
        const _: () = assert!(
            true,
            "std::is_same_v<decltype(acc.offset(ptr, i)), typename std::default_accessor<T>::data_handle_type>"
        );;
        const _: () = assert!(true, "");;
        unimplemented!();
        i.postfix_inc();
    }
}
pub unsafe fn test_4() -> () {
    (unsafe { test_offset_0() });
    (unsafe { test_offset_1() });
    (unsafe { test_offset_2() });
    (unsafe { test_offset_3() });
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
    (unsafe { test_4() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
