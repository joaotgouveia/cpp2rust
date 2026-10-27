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
pub unsafe fn operator_sub_4(
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
pub unsafe fn test_5(mut s: (), mut pos1: (), mut n1: (), mut str: *const (), mut expected: ()) {
    let mut old_size: () = unimplemented!();
    let mut first: () = unimplemented!();
    let mut last: () = unimplemented!();
    let mut xlen: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut rlen: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test_6(mut s: (), mut pos1: (), mut n1: (), mut str: *const (), mut expected: ()) {
    let mut old_size: () = unimplemented!();
    let mut first: () = unimplemented!();
    let mut last: () = unimplemented!();
    let mut xlen: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    let mut rlen: () = unimplemented!();
    unimplemented!();
}
pub unsafe fn test0_7() -> () {
    (unsafe { test_5(unimplemented!(), 0_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 0_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 0_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 0_(), 2_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 0_(), 4_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 0_(), 5_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 1_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 1_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 1_(), 2_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 1_(), 3_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 1_(), 4_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 2_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 2_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 2_(), 2_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 2_(), 3_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 4_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 4_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 5_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 0_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 0_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 0_(), 5_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 0_(), 9_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            10_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 1_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 1_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test0_8() -> () {
    (unsafe { test_6(unimplemented!(), 0_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 0_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 0_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 0_(), 2_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 0_(), 4_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 0_(), 5_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 1_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 1_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 1_(), 2_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 1_(), 3_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 1_(), 4_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 2_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 2_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 2_(), 2_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 2_(), 3_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 4_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 4_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 5_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 0_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 0_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 0_(), 5_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 0_(), 9_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 1_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 1_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test1_9() -> () {
    (unsafe { test_5(unimplemented!(), 1_(), 4_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 1_(), 8_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 1_(), 9_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 5_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 5_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 5_(), 2_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 5_(), 4_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 5_(), 5_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 9_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 9_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            0_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 0_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 0_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            10_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            19_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            20_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 1_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 1_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_5(unimplemented!(), 1_(), 9_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            18_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            19_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            0_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            1_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            5_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            9_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test1_10() -> () {
    (unsafe { test_6(unimplemented!(), 1_(), 4_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 1_(), 8_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 1_(), 9_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 5_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 5_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 5_(), 2_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 5_(), 4_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 5_(), 5_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 9_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 9_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 0_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 0_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 1_(), 0_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 1_(), 1_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe { test_6(unimplemented!(), 1_(), 9_(), c"".as_ptr(), unimplemented!()) });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test2_11() -> () {
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            10_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            19_(),
            0_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            19_(),
            1_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            20_(),
            0_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_5(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    {
        let mut s_short: () = unimplemented!();
        let mut s_long: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn test2_12() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            unimplemented!(),
        )
    });
    {
        let mut s_short: () = unimplemented!();
        let mut s_long: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    return true;
}
pub unsafe fn test_13() {
    (unsafe { test0_7() });
    (unsafe { test1_9() });
    (unsafe { test2_11() });
    const _: () = assert!(true, "test0<S>()");;
    const _: () = assert!(true, "test1<S>()");;
    const _: () = assert!(true, "test2<S>()");;
}
pub unsafe fn test_14() {
    (unsafe { test0_8() });
    (unsafe { test1_10() });
    (unsafe { test2_12() });
    const _: () = assert!(true, "test0<S>()");;
    const _: () = assert!(true, "test1<S>()");;
    const _: () = assert!(true, "test2<S>()");;
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
    (unsafe { test_13() });
    (unsafe { test_14() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
