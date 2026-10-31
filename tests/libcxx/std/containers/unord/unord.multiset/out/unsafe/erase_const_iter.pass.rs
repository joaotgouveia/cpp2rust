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
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_usize;
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_3: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl TemplateConstructor {
    pub unsafe fn new(_a0: *const i32) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(i32, new, ptr)]
pub struct TemplateConstructor {}
impl std::cmp::PartialEq for TemplateConstructor {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_4(
                self as *const TemplateConstructor,
                other as *const TemplateConstructor,
            )
        }
    }
}
impl std::cmp::Eq for TemplateConstructor {}
pub unsafe fn operator_eq_4(
    _a0: *const TemplateConstructor,
    _a1: *const TemplateConstructor,
) -> bool {
    return false;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Hash {}
impl Hash {
    pub unsafe fn operator_call(&self, _a0: *const TemplateConstructor) -> usize {
        return 0_usize;
    }
}
impl Callable1<*const TemplateConstructor, usize> for Hash {
    fn call(&self, a1: *const TemplateConstructor) -> usize {
        unsafe { Hash::operator_call(self, a1) }
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
        let mut a: [i32; 6] = [1, 2, 3, 4, 1, 2];
        let mut c: () = unimplemented!();
        let mut i: () = <()>::from(unimplemented!());
        let mut i_next: () = i.clone();
        i_next.iter_inc();
        let mut j: () = unimplemented!();
        assert!(j == i_next);
        assert!(((unimplemented!()) == (5_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
    }
    {
        let mut a: [i32; 6] = [1, 2, 3, 4, 1, 2];
        let mut c: () = unimplemented!();
        let mut i: () = <()>::from(unimplemented!());
        let mut i_next: () = i.clone();
        i_next.iter_inc();
        let mut j: () = unimplemented!();
        assert!(j == i_next);
        assert!(((unimplemented!()) == (5_())));
        assert!(((unimplemented!()) == (2_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
        assert!(((unimplemented!()) == (1_())));
    }
    {
        let mut m: () = unimplemented!();
        let mut a: TemplateConstructor = {
            let mut __tmp_0: i32 = 0;
            TemplateConstructor::new({ &mut __tmp_0 })
        };
        let mut it: () = unimplemented!();
        if it != unimplemented!() {
            unimplemented!();
        }
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
