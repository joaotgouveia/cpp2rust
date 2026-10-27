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
pub unsafe fn operator_eq_5(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_6(
    mut x: min_pointer_void__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_void__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_void__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_5(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
impl std::cmp::PartialEq for min_pointer_void__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_5(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
pub unsafe fn do_remove_7(l: *mut (), value: *const (), mut expected: ()) {
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn do_remove_8(l: *mut (), value: *const S, mut expected: ()) {
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
pub unsafe fn do_remove_9(l: *mut (), value: *const (), mut expected: ()) {
    let mut old_size: () = unimplemented!();
    const _: () = assert!(true, "");;
    unimplemented!();
    unimplemented!();
}
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe, Default)]
pub struct S {
    pub i_: *mut (),
}
impl S {
    pub unsafe fn new(mut i: ()) -> Self {
        let mut this = Self {
            i_: (Box::leak(Box::new(i)) as *mut ()),
        };
        this
    }
    pub unsafe fn copy_from(rhs: *const S) -> Self {
        let mut this = Self {
            i_: (Box::leak(Box::new((*(*rhs).i_))) as *mut ()),
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
    pub unsafe fn operator_eq(&self, rhs: *const S) -> () {
        return ((*self.i_) == (*(*rhs).i_));
    }
    pub unsafe fn get(&self) -> () {
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
pub unsafe fn test_10() -> () {
    {
        let t1: [(); 7] = [0_(), 5_(), 5_(), 0_(), 0_(), 0_(), 5_()];
        let t2: [(); 3] = [5_(), 5_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe {
            let mut _value: () = 0_();
            do_remove_7(&mut c1, &mut _value, 4_())
        });
        unimplemented!();
    }
    {
        let t1: [(); 4] = [0_(), 0_(), 0_(), 0_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe {
            let mut _value: () = 0_();
            do_remove_7(&mut c1, &mut _value, 4_())
        });
        unimplemented!();
    }
    {
        let t1: [(); 3] = [5_(), 5_(), 5_()];
        let t2: [(); 3] = [5_(), 5_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe {
            let mut _value: () = 0_();
            do_remove_7(&mut c1, &mut _value, 0_())
        });
        unimplemented!();
    }
    {
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe {
            let mut _value: () = 0_();
            do_remove_7(&mut c1, &mut _value, 0_())
        });
        unimplemented!();
    }
    {
        let t1: [(); 4] = [5_(), 5_(), 5_(), 0_()];
        let t2: [(); 3] = [5_(), 5_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe {
            let mut _value: () = 0_();
            do_remove_7(&mut c1, &mut _value, 1_())
        });
        unimplemented!();
    }
    {
        let mut t1: [(); 7] = [1_(), 2_(), 1_(), 3_(), 5_(), 8_(), 11_()];
        let mut t2: [(); 5] = [2_(), 3_(), 5_(), 8_(), 11_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe {
            let _l: *mut () = &mut c1;
            let _value: *const () = unimplemented!();
            do_remove_7(_l, _value, 2_())
        });
        unimplemented!();
    }
    {
        let mut t1: [(); 8] = [1_(), 2_(), 1_(), 3_(), 5_(), 8_(), 11_(), 1_()];
        let mut t2: [(); 5] = [2_(), 3_(), 5_(), 8_(), 11_()];
        let mut c: () = unimplemented!();
        let mut ip: *mut () = unimplemented!();
        'loop_: while ((ip) != (unimplemented!())) {
            unimplemented!();
        }
        (unsafe {
            let _l: *mut () = &mut c;
            let _value: *const S = unimplemented!();
            do_remove_8(_l, _value, 3_())
        });
        let mut it: () = unimplemented!();
        let mut ip: *mut () = unimplemented!();
        'loop_: while ((ip) != (unimplemented!())) {
            unimplemented!();
            unimplemented!();
            {
                ip.prefix_inc();
                unimplemented!()
            };
        }
        unimplemented!();
    }
    {
        let t1: [(); 7] = [0_(), 5_(), 5_(), 0_(), 0_(), 0_(), 5_()];
        let t2: [(); 3] = [5_(), 5_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe {
            let mut _value: () = 0_();
            do_remove_9(&mut c1, &mut _value, 4_())
        });
        unimplemented!();
    }
    {
        let t1: [(); 4] = [0_(), 0_(), 0_(), 0_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe {
            let mut _value: () = 0_();
            do_remove_9(&mut c1, &mut _value, 4_())
        });
        unimplemented!();
    }
    {
        let t1: [(); 3] = [5_(), 5_(), 5_()];
        let t2: [(); 3] = [5_(), 5_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe {
            let mut _value: () = 0_();
            do_remove_9(&mut c1, &mut _value, 0_())
        });
        unimplemented!();
    }
    {
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe {
            let mut _value: () = 0_();
            do_remove_9(&mut c1, &mut _value, 0_())
        });
        unimplemented!();
    }
    {
        let t1: [(); 4] = [5_(), 5_(), 5_(), 0_()];
        let t2: [(); 3] = [5_(), 5_(), 5_()];
        let mut c1: () = unimplemented!();
        let mut c2: () = unimplemented!();
        (unsafe {
            let mut _value: () = 0_();
            do_remove_9(&mut c1, &mut _value, 1_())
        });
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
