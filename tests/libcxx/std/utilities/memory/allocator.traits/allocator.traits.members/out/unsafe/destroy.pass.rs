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
pub struct NoDestroy_CountDestructor_ {}
impl NoDestroy_CountDestructor_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut CountDestructor {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut CountDestructor, mut n: usize) {
        unimplemented!();
        return;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct NoDestroy_Holder_NS_Incomplete__ptr_ {}
impl NoDestroy_Holder_NS_Incomplete__ptr_ {
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut *mut Holder_NS_Incomplete_ {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut *mut Holder_NS_Incomplete_, mut n: usize) {
        unimplemented!();
        return;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct CountDestroy_CountDestructor_ {
    pub counter_: *mut i32,
}
impl CountDestroy_CountDestructor_ {
    pub unsafe fn new(mut counter: *mut i32) -> Self {
        let mut this = Self { counter_: counter };
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut CountDestructor {
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut CountDestructor, mut n: usize) {
        unimplemented!();
        return;
    }
    pub unsafe fn destroy_4_struct_CountDestructor(&mut self, mut p: *mut CountDestructor) {
        (*self.counter_).prefix_inc();
        (unsafe { CountDestructor::destructor(&mut (*p)) });
    }
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct CountDestructor {
    pub counter_: *mut i32,
}
impl CountDestructor {
    pub unsafe fn new(mut counter: *mut i32) -> Self {
        let mut this = Self { counter_: counter };
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*self.counter_).prefix_inc();
    }
}
pub unsafe fn test_4() -> bool {
    {
        let mut destructors: i32 = 0;
        let mut alloc: NoDestroy_CountDestructor_ = <NoDestroy_CountDestructor_>::default();
        let mut pool: *mut CountDestructor = unimplemented!();
        unimplemented!();
        assert!(((destructors) == (0)));
        unimplemented!();
        assert!(((destructors) == (1)));
        unimplemented!();
    }
    {
        let mut alloc: NoDestroy_Holder_NS_Incomplete__ptr_ =
            <NoDestroy_Holder_NS_Incomplete__ptr_>::default();
        let mut pool: *mut *mut Holder_NS_Incomplete_ = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut destroys_called: i32 = 0;
        let mut destructors_called: i32 = 0;
        let mut alloc: CountDestroy_CountDestructor_ =
            CountDestroy_CountDestructor_::new({ (&mut destroys_called as *mut i32) });
        let mut pool: *mut CountDestructor = unimplemented!();
        unimplemented!();
        assert!(((destroys_called) == (0)));
        assert!(((destructors_called) == (0)));
        unimplemented!();
        assert!(((destroys_called) == (1)));
        assert!(((destructors_called) == (1)));
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_4() });
    const _: () = assert!(true, "test()");;
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct Holder_NS_Incomplete_;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
