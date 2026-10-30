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
pub struct soccc_allocator_char_ {
    soccc_count: *mut (),
    self_soccc_count: (),
}
impl soccc_allocator_char_ {
    pub unsafe fn new(mut soccc_count_: *mut (), mut self_coccc_count_: Option<()>) -> Self {
        let mut self_coccc_count_: () = self_coccc_count_.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self {
            soccc_count: soccc_count_,
            self_soccc_count: self_coccc_count_,
        };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
    }
    pub unsafe fn select_on_container_copy_construction(&self) -> soccc_allocator_char_ {
        (*self.soccc_count) += 1_();
        return soccc_allocator_char_::new({ self.soccc_count }, {
            Some(((self.self_soccc_count) + (1_())))
        });
    }
    pub unsafe fn get_soccc(&self) -> *mut () {
        return self.soccc_count;
    }
    pub unsafe fn get_self_soccc(&self) -> () {
        return self.self_soccc_count;
    }
    pub unsafe fn operator_eq_8_char(&self, that: *const soccc_allocator_char_) -> () {
        return ((self.soccc_count) == (unsafe { soccc_allocator_char_::get_soccc(&(*that)) }));
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct soccc_allocator_wchar_t_ {
    soccc_count: *mut (),
    self_soccc_count: (),
}
impl soccc_allocator_wchar_t_ {
    pub unsafe fn new(mut soccc_count_: *mut (), mut self_coccc_count_: Option<()>) -> Self {
        let mut self_coccc_count_: () = self_coccc_count_.unwrap_or_else(|| unsafe { 0_() });
        let mut this = Self {
            soccc_count: soccc_count_,
            self_soccc_count: self_coccc_count_,
        };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        unimplemented!();
    }
    pub unsafe fn select_on_container_copy_construction(&self) -> soccc_allocator_wchar_t_ {
        (*self.soccc_count) += 1_();
        return soccc_allocator_wchar_t_::new({ self.soccc_count }, {
            Some(((self.self_soccc_count) + (1_())))
        });
    }
    pub unsafe fn get_soccc(&self) -> *mut () {
        return self.soccc_count;
    }
    pub unsafe fn get_self_soccc(&self) -> () {
        return self.self_soccc_count;
    }
    pub unsafe fn operator_eq_8_wchar_t(&self, that: *const soccc_allocator_wchar_t_) -> () {
        return ((self.soccc_count) == (unsafe { soccc_allocator_wchar_t_::get_soccc(&(*that)) }));
    }
}
pub unsafe fn test_0() -> () {
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn test_1() -> () {
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut soccc_lhs: () = 0_();
        let mut soccc_rhs: () = 0_();
        let mut lhs: () = unimplemented!();
        let mut rhs: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
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
    (unsafe { test_0() });
    (unsafe { test_1() });
    const _: () = assert!(true, "test<char>()");;
    const _: () = assert!(true, "test<wchar_t>()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
