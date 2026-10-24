extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn DoNotOptimize_0(value: *const ()) -> *const () {
    c"";
    return value;
}
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
pub unsafe fn operator_sub_5(
    mut x: min_pointer_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_char__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
    pub unsafe fn pointer_to(
        t: *mut (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_6(
    mut x: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *const ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut p: min_pointer_char__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_arrow(&self) -> *const () {
        return self.ptr_;
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self
            as *mut min_pointer_const_char__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn pointer_to(
        t: *const (),
    ) -> min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_const_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_char_ {}
impl min_allocator_char_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        return min_pointer_char__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_char__std_integral_constant_unsigned_long__0__,
        mut n: (),
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_char_ {}
impl safe_allocator_char_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        let mut memory: *mut () = unimplemented!();
        {
            (unsafe {
                memset_7(
                    (memory as ()),
                    0_(),
                    (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                )
            });
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut n: ()) {
        {
            (unsafe {
                let mut _value: () = (unsafe {
                    memset_7(
                        (p as ()),
                        0_(),
                        (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                    )
                });
                DoNotOptimize_0(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl Default for safe_allocator_char_ {
    fn default() -> Self {
        unsafe { safe_allocator_char_::new() }
    }
}
pub unsafe fn test_8(mut s: (), mut pos: (), mut n: (), mut c: (), mut expected: ()) {
    let mut p: () = unimplemented!();
    let mut i: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_9(mut s: (), mut pos: (), mut n: (), mut c: (), mut expected: ()) {
    let mut p: () = unimplemented!();
    let mut i: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_10(mut s: (), mut pos: (), mut n: (), mut c: (), mut expected: ()) {
    let mut p: () = unimplemented!();
    let mut i: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_11() {
    (unsafe { test_8(unimplemented!(), 0_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 1_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 1_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 1_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 1_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 2_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 2_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 2_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 2_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 4_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 4_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 4_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 4_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 5_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 5_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 5_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 5_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 1_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 1_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 1_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 1_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 5_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 5_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 5_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 5_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 9_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 9_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 9_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 9_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 10_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 10_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            20_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe { test_8(unimplemented!(), 0_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 0_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 1_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 1_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 1_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 1_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 10_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 10_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            20_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe { test_8(unimplemented!(), 19_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 19_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            10_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            20_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe { test_8(unimplemented!(), 20_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_8(unimplemented!(), 20_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            10_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            20_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_string_12() {
    (unsafe { test_9(unimplemented!(), 0_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 1_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 1_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 1_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 1_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 2_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 2_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 2_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 2_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 4_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 4_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 4_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 4_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 5_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 5_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 5_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 5_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 1_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 1_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 1_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 1_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 5_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 5_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 5_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 5_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 9_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 9_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 9_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 9_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 10_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 10_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe {
        test_9(
            unimplemented!(),
            10_(),
            10_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            10_(),
            20_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe { test_9(unimplemented!(), 0_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 0_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 1_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 1_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 1_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 1_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 10_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 10_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe {
        test_9(
            unimplemented!(),
            10_(),
            10_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            10_(),
            20_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe { test_9(unimplemented!(), 19_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 19_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe {
        test_9(
            unimplemented!(),
            19_(),
            10_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            19_(),
            20_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe { test_9(unimplemented!(), 20_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_9(unimplemented!(), 20_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe {
        test_9(
            unimplemented!(),
            20_(),
            10_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            20_(),
            20_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_string_13() {
    (unsafe { test_10(unimplemented!(), 0_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 1_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 1_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 1_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 1_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 2_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 2_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 2_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 2_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 4_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 4_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 4_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 4_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 5_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 5_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 5_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 5_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 1_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 1_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 1_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 1_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 5_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 5_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 5_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 5_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 9_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 9_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 9_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 9_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 10_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 10_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe {
        test_10(
            unimplemented!(),
            10_(),
            10_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            10_(),
            20_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe { test_10(unimplemented!(), 0_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 0_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 1_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 1_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 1_(), 10_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 1_(), 20_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 10_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 10_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe {
        test_10(
            unimplemented!(),
            10_(),
            10_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            10_(),
            20_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe { test_10(unimplemented!(), 19_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 19_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe {
        test_10(
            unimplemented!(),
            19_(),
            10_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            19_(),
            20_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe { test_10(unimplemented!(), 20_(), 0_(), ('1' as ()), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), 20_(), 5_(), ('1' as ()), unimplemented!()) });
    (unsafe {
        test_10(
            unimplemented!(),
            20_(),
            10_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            20_(),
            20_(),
            ('1' as ()),
            unimplemented!(),
        )
    });
}
pub unsafe fn test_14() -> () {
    (unsafe { test_string_11() });
    (unsafe { test_string_12() });
    (unsafe { test_string_13() });
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
    (unsafe { test_14() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
