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
pub struct sized_allocator_bool__unsigned_char__signed_char_ {
    data_: i32,
}
impl sized_allocator_bool__unsigned_char__signed_char_ {
    pub unsafe fn new(mut d: i32) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_char__signed_char_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(sized_allocator_bool__unsigned_char__signed_char_, new_1, ptr)]
pub struct sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_char__signed_char_ {
    pub unsafe fn allocate(&mut self, mut n: u8) -> *mut u64 {
        if ((n as i32)
            > ((unsafe {
                sized_allocator_unsigned_long__unsigned_char__signed_char_::max_size(self)
            }) as i32))
        {
            std::process::abort();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: u8) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u8 {
        return (((u8::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize))) as u8);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_short__short_ {
    data_: i32,
}
impl sized_allocator_bool__unsigned_short__short_ {
    pub unsafe fn new(mut d: i32) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_long__unsigned_short__short_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_short__short_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(sized_allocator_bool__unsigned_short__short_, new_1, ptr)]
pub struct sized_allocator_unsigned_long__unsigned_short__short_ {
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_short__short_ {
    pub unsafe fn allocate(&mut self, mut n: u16) -> *mut u64 {
        if ((n as i32)
            > ((unsafe { sized_allocator_unsigned_long__unsigned_short__short_::max_size(self) })
                as i32))
        {
            std::process::abort();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: u16) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u16 {
        return (((u16::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u16);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_int__int_ {
    data_: i32,
}
impl sized_allocator_bool__unsigned_int__int_ {
    pub unsafe fn new(mut d: i32) -> Self {
        let mut this = Self { data_: d };
        this
    }
}
impl sized_allocator_unsigned_long__unsigned_int__int_ {
    pub unsafe fn new_1(a: *const sized_allocator_bool__unsigned_int__int_) -> Self {
        let mut this = Self { data_: (*a).data_ };
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(sized_allocator_bool__unsigned_int__int_, new_1, ptr)]
pub struct sized_allocator_unsigned_long__unsigned_int__int_ {
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_int__int_ {
    pub unsafe fn allocate(&mut self, mut n: u32) -> *mut u64 {
        if ((n) > (unsafe { sized_allocator_unsigned_long__unsigned_int__int_::max_size(self) })) {
            std::process::abort();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: u32) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u32 {
        return (((u32::MAX as usize).wrapping_div((::std::mem::size_of::<u64>() as usize)))
            as u32);
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct sized_allocator_bool__unsigned_long__long_ {
    data_: i32,
}
impl sized_allocator_bool__unsigned_long__long_ {
    pub unsafe fn new(mut d: i32) -> Self {
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
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(sized_allocator_bool__unsigned_long__long_, new_1, ptr)]
pub struct sized_allocator_unsigned_long__unsigned_long__long_ {
    data_: i32,
}
impl sized_allocator_unsigned_long__unsigned_long__long_ {
    pub unsafe fn allocate(&mut self, mut n: u64) -> *mut u64 {
        if ((n) > (unsafe { sized_allocator_unsigned_long__unsigned_long__long_::max_size(self) }))
        {
            std::process::abort();
        }
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut u64, mut n: u64) {
        unimplemented!();
    }
    pub unsafe fn max_size(&self) -> u64 {
        return (u64::MAX as u64).wrapping_div((::std::mem::size_of::<u64>() as u64));
    }
}
pub unsafe fn tests_0() -> bool {
    {
        let mut c: Vec<bool> = unimplemented!();
        {
            let __a0 = 10_usize as usize;
            c.resize_with(__a0, || false)
        };
        assert!((((&c).len()) == (10_usize)));
        assert!(((unimplemented!()) >= (10_usize)));
    }
    {
        let mut c: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!((((&c).len()) == (10_usize)));
        assert!(((unimplemented!()) >= (10_usize)));
    }
    {
        let mut c: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!((((&c).len()) == (1_usize)));
        assert!(((unimplemented!()) >= (1_usize)));
    }
    {
        let mut c: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!((((&c).len()) == (10_usize)));
        assert!(((unimplemented!()) >= (10_usize)));
    }
    {
        let mut c: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!((((&c).len()) == (1_usize)));
        assert!(((unimplemented!()) >= (1_usize)));
    }
    {
        let mut c: Vec<bool> = unimplemented!();
        unimplemented!();
        assert!((((&c).len()) == (10_usize)));
        assert!(((unimplemented!()) >= (10_usize)));
    }
    {
        let mut c: Vec<bool> = unimplemented!();
        {
            let __a0 = 10_usize as usize;
            c.resize_with(__a0, || false)
        };
        assert!((((&c).len()) == (10_usize)));
        assert!(((unimplemented!()) >= (10_usize)));
    }
    {
        let mut c: Vec<bool> = unimplemented!();
        {
            let __a0 = 10_usize as usize;
            c.resize_with(__a0, || false)
        };
        assert!((((&c).len()) == (10_usize)));
        assert!(((unimplemented!()) >= (10_usize)));
    }
    {
        let mut c: Vec<bool> = unimplemented!();
        {
            let __a0 = 10_usize as usize;
            c.resize_with(__a0, || false)
        };
        assert!((((&c).len()) == (10_usize)));
        assert!(((unimplemented!()) >= (10_usize)));
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
    (unsafe { tests_0() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
