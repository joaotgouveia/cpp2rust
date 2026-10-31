extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
struct no_default_allocator_int__construct_tag {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct no_default_allocator_int_ {}
impl no_default_allocator_int_ {
    unsafe fn new_1(mut _a0: no_default_allocator_int__construct_tag) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn create() -> no_default_allocator_int_ {
        let mut tag: no_default_allocator_int__construct_tag =
            <no_default_allocator_int__construct_tag>::default();
        return no_default_allocator_int_::new_1({ tag });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_4)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_1)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3)) = 0_usize;
    }
}
pub static mut outstanding_bytes_1: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_2: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_3: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_4: std::cell::LazyCell<bool> =
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
#[repr(C)]
#[derive(VaArg, FnPtrArg, CppFromUnsafe, CopyAssignUnsafe, DestructorUnsafe, Default)]
#[cpp_from(i32, new, value)]
pub struct S {
    pub i_: *mut i32,
}
impl S {
    pub unsafe fn new(mut i: i32) -> Self {
        let mut this = Self {
            i_: (Box::leak(Box::new(i)) as *mut i32),
        };
        this
    }
    pub unsafe fn copy_from(rhs: *const S) -> Self {
        let mut this = Self {
            i_: (Box::leak(Box::new((*(*rhs).i_))) as *mut i32),
        };
        this
    }
    pub unsafe fn copy_assign(&mut self, rhs: *const S) -> *mut S {
        (*self.i_) = (*(*rhs).i_);
        return &mut (*(self as *mut S));
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.i_;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
        self.i_ = std::ptr::null_mut();
    }
    pub unsafe fn operator_eq(&self, rhs: *const S) -> bool {
        return ((*self.i_) == (*(*rhs).i_));
    }
    pub unsafe fn get(&self) -> i32 {
        return (*self.i_);
    }
}
impl std::cmp::PartialEq for S {
    fn eq(&self, other: &Self) -> bool {
        unsafe { S::operator_eq(self, other as *const S) }
    }
}
impl std::cmp::Eq for S {}
impl Clone for S {
    fn clone(&self) -> Self {
        unsafe { S::copy_from(self as *const S) }
    }
}
pub unsafe fn test_5() -> bool {
    {
        let mut a1: [i32; 4] = [1, 2, 3, 4];
        let mut a2: [i32; 3] = [1, 2, 4];
        let mut c: () = unimplemented!();
        assert!(((unimplemented!()) == (1_())));
        const _: () = assert!(true, "");;
        assert!(unimplemented!());
    }
    {
        let mut a1: [i32; 7] = [1, 2, 1, 3, 5, 8, 11];
        let mut a2: [i32; 5] = [2, 3, 5, 8, 11];
        let mut c: () = unimplemented!();
        unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut a1: [i32; 8] = [1, 2, 1, 3, 5, 8, 11, 1];
        let mut a2: [i32; 5] = [2, 3, 5, 8, 11];
        let mut c: () = unimplemented!();
        let mut ip: *mut i32 = a1.as_mut_ptr();
        'loop_: while ((ip) < (a1.as_mut_ptr().offset((8) as isize))) {
            unimplemented!();
            ip.prefix_inc();
        }
        assert!(((unimplemented!()) == (3_())));
        let mut it: () = <UnsafeBidirectionalIterator<S>>::from(unimplemented!());
        let mut ip: *mut i32 = a2.as_mut_ptr();
        'loop_: while ((ip) < (a2.as_mut_ptr().offset((5) as isize))) {
            assert!(it != <UnsafeBidirectionalIterator<S>>::from(unimplemented!()));
            assert!(((*ip) == (unsafe { S::get(&(*(it.iter_deref() as *const S)),) })));
            {
                ip.prefix_inc();
                it.iter_inc()
            };
        }
        assert!(it == <UnsafeBidirectionalIterator<S>>::from(unimplemented!()));
    }
    {
        let mut a1: [i32; 4] = [1, 2, 3, 4];
        let mut a2: [i32; 3] = [1, 2, 4];
        let mut c: () = unimplemented!();
        assert!(((unimplemented!()) == (1_())));
        assert!(unimplemented!());
    }
    {
        let mut a1: [i32; 4] = [1, 2, 3, 4];
        let mut a2: [i32; 3] = [1, 2, 4];
        let mut c: () = unimplemented!();
        assert!(((unimplemented!()) == (1_())));
        assert!((unimplemented!()));
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
    assert!((unsafe { test_5() }));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
