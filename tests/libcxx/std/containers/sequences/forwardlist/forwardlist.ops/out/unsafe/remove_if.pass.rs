extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_unsigned_long__unsigned_long_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_4)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_1)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_2)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_3)) = 0_();
    }
}
pub static mut outstanding_bytes_1: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_2: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct unary_counting_predicate_bool__ptr__int___int_ {
    p_: Option<unsafe fn(()) -> ()>,
    count_: (),
}
impl unary_counting_predicate_bool__ptr__int___int_ {
    pub unsafe fn new(mut p: Option<unsafe fn(()) -> ()>) -> Self {
        let mut this = Self {
            p_: p,
            count_: 0_(),
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
    pub unsafe fn operator_call(&self, a: *const ()) -> () {
        (*(&raw const self.count_).cast_mut()).prefix_inc();
        return (unsafe { (self.p_).unwrap()((*a)) });
    }
    pub unsafe fn count(&self) -> () {
        return self.count_;
    }
}
pub unsafe fn do_remove_if_5(l: *mut (), mut pred: (), mut expected: ()) {
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn do_remove_if_6(l: *mut (), mut pred: (), mut expected: ()) {
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn do_remove_if_7(l: *mut (), mut pred: (), mut expected: ()) {
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn g_8(mut i: ()) -> () {
    return ((i) < (3_()));
}
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct PredLWG526 {
    pub i_: (),
}
impl PredLWG526 {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn destructor(&mut self) {
        self.i_ = -32767_();
    }
    pub unsafe fn operator_call(&self, p: *const PredLWG526) -> () {
        return (((*p).i_) == (self.i_));
    }
    pub unsafe fn operator_eq(&self, mut i: ()) -> () {
        return ((i) == (self.i_));
    }
}
pub unsafe fn test_9() -> () {
    {
        let t1: [(); 7] = [0_(), 5_(), 5_(), 0_(), 0_(), 0_(), 5_()];
        let t2: [(); 3] = [5_(), 5_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        let mut cp: unary_counting_predicate_bool__ptr__int___int_ =
            unary_counting_predicate_bool__ptr__int___int_::new({ Some(g_8) });
        let _dtor_cp = ScopedDestructorUnsafe::new(
            &raw mut cp,
            unary_counting_predicate_bool__ptr__int___int_::destructor,
        );
        (unsafe { do_remove_if_5(&mut c1, unimplemented!(), 4_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let t1: [(); 4] = [0_(), 0_(), 0_(), 0_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        let mut cp: unary_counting_predicate_bool__ptr__int___int_ =
            unary_counting_predicate_bool__ptr__int___int_::new({ Some(g_8) });
        let _dtor_cp = ScopedDestructorUnsafe::new(
            &raw mut cp,
            unary_counting_predicate_bool__ptr__int___int_::destructor,
        );
        (unsafe { do_remove_if_5(&mut c1, unimplemented!(), 4_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let t1: [(); 3] = [5_(), 5_(), 5_()];
        let t2: [(); 3] = [5_(), 5_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        let mut cp: unary_counting_predicate_bool__ptr__int___int_ =
            unary_counting_predicate_bool__ptr__int___int_::new({ Some(g_8) });
        let _dtor_cp = ScopedDestructorUnsafe::new(
            &raw mut cp,
            unary_counting_predicate_bool__ptr__int___int_::destructor,
        );
        (unsafe { do_remove_if_5(&mut c1, unimplemented!(), 0_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        let mut cp: unary_counting_predicate_bool__ptr__int___int_ =
            unary_counting_predicate_bool__ptr__int___int_::new({ Some(g_8) });
        let _dtor_cp = ScopedDestructorUnsafe::new(
            &raw mut cp,
            unary_counting_predicate_bool__ptr__int___int_::destructor,
        );
        (unsafe { do_remove_if_5(&mut c1, unimplemented!(), 0_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let t1: [(); 4] = [5_(), 5_(), 5_(), 0_()];
        let t2: [(); 3] = [5_(), 5_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        let mut cp: unary_counting_predicate_bool__ptr__int___int_ =
            unary_counting_predicate_bool__ptr__int___int_::new({ Some(g_8) });
        let _dtor_cp = ScopedDestructorUnsafe::new(
            &raw mut cp,
            unary_counting_predicate_bool__ptr__int___int_::destructor,
        );
        (unsafe { do_remove_if_5(&mut c1, unimplemented!(), 1_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut a1: [(); 7] = [1_(), 2_(), 1_(), 3_(), 5_(), 8_(), 11_()];
        let mut a2: [(); 5] = [2_(), 3_(), 5_(), 8_(), 11_()];
        let mut c: () = unimplemented!();
        (unsafe {
            let _l: *mut () = &mut c;
            let _pred: () = unimplemented!();
            do_remove_if_6(_l, _pred, 2_())
        });
        let mut i: () = 0_();
        'loop_: while ((i) < (5_())) {
            unimplemented!();
            unimplemented!();
            unimplemented!();
            i.prefix_inc();
        }
        unimplemented!();
    }
    {
        let t1: [(); 7] = [0_(), 5_(), 5_(), 0_(), 0_(), 0_(), 5_()];
        let t2: [(); 3] = [5_(), 5_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        let mut cp: unary_counting_predicate_bool__ptr__int___int_ =
            unary_counting_predicate_bool__ptr__int___int_::new({ Some(g_8) });
        let _dtor_cp = ScopedDestructorUnsafe::new(
            &raw mut cp,
            unary_counting_predicate_bool__ptr__int___int_::destructor,
        );
        (unsafe { do_remove_if_7(&mut c1, unimplemented!(), 4_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let t1: [(); 4] = [0_(), 0_(), 0_(), 0_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        let mut cp: unary_counting_predicate_bool__ptr__int___int_ =
            unary_counting_predicate_bool__ptr__int___int_::new({ Some(g_8) });
        let _dtor_cp = ScopedDestructorUnsafe::new(
            &raw mut cp,
            unary_counting_predicate_bool__ptr__int___int_::destructor,
        );
        (unsafe { do_remove_if_7(&mut c1, unimplemented!(), 4_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let t1: [(); 3] = [5_(), 5_(), 5_()];
        let t2: [(); 3] = [5_(), 5_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        let mut cp: unary_counting_predicate_bool__ptr__int___int_ =
            unary_counting_predicate_bool__ptr__int___int_::new({ Some(g_8) });
        let _dtor_cp = ScopedDestructorUnsafe::new(
            &raw mut cp,
            unary_counting_predicate_bool__ptr__int___int_::destructor,
        );
        (unsafe { do_remove_if_7(&mut c1, unimplemented!(), 0_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        let mut cp: unary_counting_predicate_bool__ptr__int___int_ =
            unary_counting_predicate_bool__ptr__int___int_::new({ Some(g_8) });
        let _dtor_cp = ScopedDestructorUnsafe::new(
            &raw mut cp,
            unary_counting_predicate_bool__ptr__int___int_::destructor,
        );
        (unsafe { do_remove_if_7(&mut c1, unimplemented!(), 0_()) });
        unimplemented!();
        unimplemented!();
    }
    {
        let t1: [(); 4] = [5_(), 5_(), 5_(), 0_()];
        let t2: [(); 3] = [5_(), 5_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        let mut cp: unary_counting_predicate_bool__ptr__int___int_ =
            unary_counting_predicate_bool__ptr__int___int_::new({ Some(g_8) });
        let _dtor_cp = ScopedDestructorUnsafe::new(
            &raw mut cp,
            unary_counting_predicate_bool__ptr__int___int_::destructor,
        );
        (unsafe { do_remove_if_7(&mut c1, unimplemented!(), 1_()) });
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
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
