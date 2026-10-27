extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) == ((*RHS).m_id));
}
pub unsafe fn operator_ne_1(LHS: *const TypeID, RHS: *const TypeID) -> () {
    return (((*LHS).m_id) != ((*RHS).m_id));
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TypeID {
    m_id: *const (),
}
impl TypeID {
    pub unsafe fn name(&self) -> () {
        return unimplemented!();
    }
    unsafe fn new(mut xid: *const ()) -> Self {
        let mut this = Self { m_id: xid };
        this
    }
}
impl std::cmp::PartialEq for TypeID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_0(self as *const TypeID, other as *const TypeID) }
    }
}
impl std::cmp::Eq for TypeID {}
pub type UsesAllocatorType = ();
pub const UsesAllocatorType_UA_None: UsesAllocatorType = 0;
pub const UsesAllocatorType_UA_AllocArg: UsesAllocatorType = 2;
pub const UsesAllocatorType_UA_AllocLast: UsesAllocatorType = 4;
pub static mut UA_None_2: std::cell::LazyCell<UsesAllocatorType> =
    std::cell::LazyCell::new(|| unsafe { UsesAllocatorType_UA_None });
pub static mut UA_AllocArg_3: std::cell::LazyCell<UsesAllocatorType> =
    std::cell::LazyCell::new(|| unsafe { UsesAllocatorType_UA_AllocArg });
pub static mut UA_AllocLast_4: std::cell::LazyCell<UsesAllocatorType> =
    std::cell::LazyCell::new(|| unsafe { UsesAllocatorType_UA_AllocLast });
pub unsafe fn toString_5(mut UA: UsesAllocatorType) -> *const () {
    switch!(match UA {
        __v if __v == (*std::cell::LazyCell::force_mut(&mut *&raw mut UA_None_2)) => {
            return c"UA_None".as_ptr();
        }
        __v if __v == (*std::cell::LazyCell::force_mut(&mut *&raw mut UA_AllocArg_3)) => {
            return c"UA_AllocArg".as_ptr();
        }
        __v if __v == (*std::cell::LazyCell::force_mut(&mut *&raw mut UA_AllocLast_4)) => {
            return c"UA_AllocLast".as_ptr();
        }
        _ => {
            unimplemented!();
        }
    });
    panic!("ub: non-void function does not return a value")
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct AllocLastTag {}
pub static mut constructed_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct default_constructible_0_ {
    pub x: (),
}
impl default_constructible_0_ {
    pub unsafe fn new() -> Self {
        let mut this = Self { x: 42_() };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)).prefix_inc();
        this
    }
}
impl Default for default_constructible_0_ {
    fn default() -> Self {
        unsafe { default_constructible_0_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct default_constructible_1_ {
    pub x: (),
}
impl default_constructible_1_ {
    pub unsafe fn new() -> Self {
        let mut this = Self { x: 42_() };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)).prefix_inc();
        this
    }
}
impl Default for default_constructible_1_ {
    fn default() -> Self {
        unsafe { default_constructible_1_::new() }
    }
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
    {
        let mut buffer: [(); 8] = [unimplemented!(); 8];
        let mut ptr: *mut () = (buffer.as_mut_ptr() as *mut ());
        let mut a: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)) = 0_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut buffer: [(); 8] = [unimplemented!(); 8];
        let mut ptr: *mut () = (buffer.as_mut_ptr() as *mut ());
        let mut a: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)) = 0_();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const UA_None_2);
    std::cell::LazyCell::force(&*&raw const UA_AllocArg_3);
    std::cell::LazyCell::force(&*&raw const UA_AllocLast_4);
    std::cell::LazyCell::force(&*&raw const constructed_6);
}
