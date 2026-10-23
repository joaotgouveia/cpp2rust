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
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_3)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_0)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_2)) = 0_();
    }
}
pub static mut outstanding_bytes_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, DestructorUnsafe, Default)]
pub struct Deleter_A_ {
    state_: (),
}
impl Deleter_A_ {
    pub unsafe fn move_from(r: *mut Deleter_A_) -> Self {
        let mut this = Self {
            state_: (*r).state_,
        };
        (*r).state_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, r: *mut Deleter_A_) -> *mut Deleter_A_ {
        self.state_ = (*r).state_;
        (*r).state_ = 0_();
        return &mut (*(self as *mut Deleter_A_));
    }
    pub unsafe fn new_1(mut s: ()) -> Self {
        let mut this = Self { state_: s };
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        self.state_ = -1_();
    }
    pub unsafe fn state(&self) -> () {
        return self.state_;
    }
    pub unsafe fn operator_call(&mut self, mut p: *mut A) {
        {
            let __p = p;
            if !__p.is_null() {
                A::destructor(&mut *__p);
                ::std::mem::drop(Box::from_raw(__p))
            }
        };
    }
}
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe, DestructorUnsafe, Default)]
pub struct Deleter_Aarrarr_ {
    state_: (),
}
impl Deleter_Aarrarr_ {
    pub unsafe fn move_from(r: *mut Deleter_Aarrarr_) -> Self {
        let mut this = Self {
            state_: (*r).state_,
        };
        (*r).state_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, r: *mut Deleter_Aarrarr_) -> *mut Deleter_Aarrarr_ {
        self.state_ = (*r).state_;
        (*r).state_ = 0_();
        return &mut (*(self as *mut Deleter_Aarrarr_));
    }
    pub unsafe fn new_1(mut s: ()) -> Self {
        let mut this = Self { state_: s };
        this
    }
    pub unsafe fn destructor(&mut self) {
        unimplemented!();
        self.state_ = -1_();
    }
    pub unsafe fn state(&self) -> () {
        return self.state_;
    }
    pub unsafe fn operator_call(&mut self, mut p: *mut A) {
        {
            let __p = p;
            if !__p.is_null() {
                for __i in 0..libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                    / ::std::mem::size_of::<A>()
                {
                    A::destructor(&mut *__p.add(__i));
                }
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<A>(),
                )))
            }
        };
    }
}
pub unsafe fn swap_4(x: *mut Deleter_A_, y: *mut Deleter_A_) {
    let mut t: Deleter_A_ = Deleter_A_::move_from({ x });
    let _dtor_t = ScopedDestructorUnsafe::new(&raw mut t, Deleter_A_::destructor);
    (unsafe {
        let _r: *mut Deleter_A_ = y;
        Deleter_A_::move_assign(&mut (*x), _r)
    });
    (unsafe {
        let _r: *mut Deleter_A_ = &mut t;
        Deleter_A_::move_assign(&mut (*y), _r)
    });
}
pub unsafe fn swap_5(x: *mut Deleter_Aarrarr_, y: *mut Deleter_Aarrarr_) {
    let mut t: Deleter_Aarrarr_ = Deleter_Aarrarr_::move_from({ x });
    let _dtor_t = ScopedDestructorUnsafe::new(&raw mut t, Deleter_Aarrarr_::destructor);
    (unsafe {
        let _r: *mut Deleter_Aarrarr_ = y;
        Deleter_Aarrarr_::move_assign(&mut (*x), _r)
    });
    (unsafe {
        let _r: *mut Deleter_Aarrarr_ = &mut t;
        Deleter_Aarrarr_::move_assign(&mut (*y), _r)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_deleter_base {}
pub static mut count_6: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_7: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn operator_eq_9(x: *const A, y: *const A) -> () {
    return (((*x).state_) == ((*y).state_));
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct A {
    pub state_: (),
}
impl A {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { state_: 0_() };
        {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut count_8)).prefix_inc();
        }
        this
    }
    pub unsafe fn new_2(mut i: ()) -> Self {
        let mut this = Self { state_: i };
        {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut count_8)).prefix_inc();
        }
        this
    }
    pub unsafe fn copy_from(a: *const A) -> Self {
        let mut this = Self {
            state_: (*a).state_,
        };
        {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut count_8)).prefix_inc();
        }
        this
    }
    pub unsafe fn copy_assign(&mut self, a: *const A) -> *mut A {
        self.state_ = (*a).state_;
        return &mut (*(self as *mut A));
    }
    pub unsafe fn destructor(&mut self) {
        {
            (*std::cell::LazyCell::force_mut(&mut *&raw mut count_8)).prefix_dec();
        }
    }
}
impl std::cmp::PartialEq for A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_9(self as *const A, other as *const A) }
    }
}
impl std::cmp::Eq for A {}
impl Clone for A {
    fn clone(&self) -> Self {
        unsafe { A::copy_from(self as *const A) }
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new_1() }
    }
}
pub static mut count_8: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
#[repr(C)]
#[derive(FnPtrArg, CopyAssignUnsafe, Default)]
pub struct NonSwappableDeleter_int_ {}
impl NonSwappableDeleter_int_ {
    pub unsafe fn new(mut _a0: ()) -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const NonSwappableDeleter_int_,
    ) -> *mut NonSwappableDeleter_int_ {
        return &mut (*(self as *mut NonSwappableDeleter_int_));
    }
    pub unsafe fn operator_call(&self, mut _a0: *mut ()) {}
}
pub unsafe fn test_10() -> () {
    {
        let mut p1: *mut A = (Box::leak(Box::new(A::new_2({ 1_() }))) as *mut A);
        let mut s1: () = unimplemented!();
        let mut p2: *mut A = (Box::leak(Box::new(A::new_2({ 2_() }))) as *mut A);
        let mut s2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
    }
    {
        unimplemented!();
    }
    {
        let mut p1: *mut A =
            Box::leak((0..3_()).map(|_| <A>::default()).collect::<Box<[A]>>()).as_mut_ptr();
        let mut s1: () = unimplemented!();
        let mut p2: *mut A =
            Box::leak((0..3_()).map(|_| <A>::default()).collect::<Box<[A]>>()).as_mut_ptr();
        let mut s2: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        {
            unimplemented!();
        }
    }
    {
        unimplemented!();
    }
    {
        let mut d: NonSwappableDeleter_int_ = NonSwappableDeleter_int_::new({ 42_() });
        let mut x: () = 42_();
        let mut y: () = 43_();
        let mut p: () = unimplemented!();
        let mut p2: () = unimplemented!();
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
    (unsafe { test_10() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const count_6);
    std::cell::LazyCell::force(&*&raw const dealloc_count_7);
    std::cell::LazyCell::force(&*&raw const count_8);
}
