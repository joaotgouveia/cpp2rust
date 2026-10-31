extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn operator_eq_0(LHS: *const TypeID, RHS: *const TypeID) -> bool {
    return (((*LHS).m_id) == ((*RHS).m_id));
}
pub unsafe fn operator_ne_1(LHS: *const TypeID, RHS: *const TypeID) -> bool {
    return (((*LHS).m_id) != ((*RHS).m_id));
}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct TypeID {
    m_id: *const libc::c_char,
}
impl TypeID {
    pub unsafe fn name(&self) -> Vec<libc::c_char> {
        return std::slice::from_raw_parts(self.m_id, libcc2rs::c_str_len(self.m_id) + 1).to_vec();
    }
    unsafe fn new(mut xid: *const libc::c_char) -> Self {
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
pub type UsesAllocatorType = i32;
pub const UsesAllocatorType_UA_None: UsesAllocatorType = 0;
pub const UsesAllocatorType_UA_AllocArg: UsesAllocatorType = 2;
pub const UsesAllocatorType_UA_AllocLast: UsesAllocatorType = 4;
pub static mut UA_None_2: std::cell::LazyCell<UsesAllocatorType> =
    std::cell::LazyCell::new(|| unsafe { UsesAllocatorType_UA_None });
pub static mut UA_AllocArg_3: std::cell::LazyCell<UsesAllocatorType> =
    std::cell::LazyCell::new(|| unsafe { UsesAllocatorType_UA_AllocArg });
pub static mut UA_AllocLast_4: std::cell::LazyCell<UsesAllocatorType> =
    std::cell::LazyCell::new(|| unsafe { UsesAllocatorType_UA_AllocLast });
pub unsafe fn toString_5(mut UA: UsesAllocatorType) -> *const libc::c_char {
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
            std::process::abort();
        }
    });
    panic!("ub: non-void function does not return a value")
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct AllocLastTag {}
pub static mut constructed_6: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct default_constructible_0_ {
    pub x: i32,
}
impl default_constructible_0_ {
    pub unsafe fn new() -> Self {
        let mut this = Self { x: 42 };
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
    pub x: i32,
}
impl default_constructible_1_ {
    pub unsafe fn new() -> Self {
        let mut this = Self { x: 42 };
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    {
        let mut buffer: [libc::c_char; 8] = [(0 as libc::c_char); 8];
        let mut ptr: *mut (default_constructible_0_, default_constructible_0_) =
            (buffer.as_mut_ptr() as *mut (default_constructible_0_, default_constructible_0_));
        let mut a: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)) = 0;
        unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)) == (2)));
        assert!((((*ptr).0.x) == (42)));
        assert!((((*ptr).1.x) == (42)));
    }
    {
        let mut buffer: [libc::c_char; 8] = [(0 as libc::c_char); 8];
        let mut ptr: *mut (default_constructible_0_, default_constructible_1_) =
            (buffer.as_mut_ptr() as *mut (default_constructible_0_, default_constructible_1_));
        let mut a: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)) = 0;
        unimplemented!();
        assert!(((*std::cell::LazyCell::force_mut(&mut *&raw mut constructed_6)) == (2)));
        assert!((((*ptr).0.x) == (42)));
        assert!((((*ptr).1.x) == (42)));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const UA_None_2);
    std::cell::LazyCell::force(&*&raw const UA_AllocArg_3);
    std::cell::LazyCell::force(&*&raw const UA_AllocLast_4);
    std::cell::LazyCell::force(&*&raw const constructed_6);
}
