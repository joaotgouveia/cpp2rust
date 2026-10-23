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
pub unsafe fn sign_4(mut x: ()) -> () {
    if ((x) == (0_())) {
        return 0_();
    }
    if ((x) < (0_())) {
        return -1_();
    }
    return 1_();
}
pub unsafe fn test_5(
    s: *const (),
    mut pos: (),
    mut n1: (),
    mut str: *const (),
    mut n2: (),
    mut x: (),
) {
    if ((pos) <= (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test_6(
    s: *const (),
    mut pos: (),
    mut n1: (),
    mut str: *const (),
    mut n2: (),
    mut x: (),
) {
    if ((pos) <= (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test0_7() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 2_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 4_(), -3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 5_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -18_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 2_(), c"".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 2_(), c"abcde".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 2_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 2_(), c"abcde".as_ptr(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 2_(), c"abcde".as_ptr(), 4_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 2_(), c"abcde".as_ptr(), 5_(), -3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 2_(), c"abcdefghij".as_ptr(), 5_(), -3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 2_(), c"abcdefghij".as_ptr(), 9_(), -7_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 2_(), c"abcdefghij".as_ptr(), 10_(), -8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -8_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -17_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -18_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 4_(), c"".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 4_(), c"abcde".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 4_(), c"abcde".as_ptr(), 1_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 4_(), c"abcde".as_ptr(), 2_(), 2_())
    });
    return true;
}
pub unsafe fn test0_8() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 2_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 4_(), -3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 5_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -18_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 2_(), c"".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 2_(), c"abcde".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 2_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 2_(), c"abcde".as_ptr(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 2_(), c"abcde".as_ptr(), 4_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 2_(), c"abcde".as_ptr(), 5_(), -3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 2_(), c"abcdefghij".as_ptr(), 5_(), -3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 2_(), c"abcdefghij".as_ptr(), 9_(), -7_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 2_(), c"abcdefghij".as_ptr(), 10_(), -8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -8_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -17_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -18_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 4_(), c"".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 4_(), c"abcde".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 4_(), c"abcde".as_ptr(), 1_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 4_(), c"abcde".as_ptr(), 2_(), 2_())
    });
    return true;
}
pub unsafe fn test1_9() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 4_(), c"abcde".as_ptr(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 4_(), c"abcde".as_ptr(), 5_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 4_(), c"abcdefghij".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 4_(), c"abcdefghij".as_ptr(), 1_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 4_(), c"abcdefghij".as_ptr(), 5_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 4_(), c"abcdefghij".as_ptr(), 9_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 4_(), c"abcdefghij".as_ptr(), 10_(), -6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            3_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -6_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -15_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -16_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 2_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 9_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 10_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -14_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -15_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 6_(), c"".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 6_(), c"abcde".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 6_(), c"abcde".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 6_(), c"abcde".as_ptr(), 2_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 6_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 6_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 6_(), c"abcdefghij".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 6_(), c"abcdefghij".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 6_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 6_(), c"abcdefghij".as_ptr(), 9_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 6_(), c"abcdefghij".as_ptr(), 10_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -14_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -15_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 2_(), c"".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 2_(), c"abcde".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 2_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 2_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 2_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 2_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 2_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 2_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 2_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 3_(), c"".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 3_(), c"abcde".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 3_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 3_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 3_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 3_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 3_(), c"abcdefghij".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 3_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    return true;
}
pub unsafe fn test1_10() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 4_(), c"abcde".as_ptr(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 4_(), c"abcde".as_ptr(), 5_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 4_(), c"abcdefghij".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 4_(), c"abcdefghij".as_ptr(), 1_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 4_(), c"abcdefghij".as_ptr(), 5_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 4_(), c"abcdefghij".as_ptr(), 9_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 4_(), c"abcdefghij".as_ptr(), 10_(), -6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            3_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -6_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -15_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -16_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 2_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 9_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 10_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -14_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -15_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 6_(), c"".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 6_(), c"abcde".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 6_(), c"abcde".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 6_(), c"abcde".as_ptr(), 2_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 6_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 6_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 6_(), c"abcdefghij".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 6_(), c"abcdefghij".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 6_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 6_(), c"abcdefghij".as_ptr(), 9_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 6_(), c"abcdefghij".as_ptr(), 10_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -14_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -15_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 2_(), c"".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 2_(), c"abcde".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 2_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 2_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 2_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 2_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 2_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 2_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 2_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 3_(), c"".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 3_(), c"abcde".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 3_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 3_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 3_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 3_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 3_(), c"abcdefghij".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 3_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    return true;
}
pub unsafe fn test2_11() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 3_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 3_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 3_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            3_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 5_(), c"".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 5_(), c"abcde".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 5_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 5_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 5_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 5_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 5_(), c"abcdefghij".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 5_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 5_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 5_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 5_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 1_(), c"abcde".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 1_(), c"abcde".as_ptr(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 1_(), c"abcde".as_ptr(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 1_(), c"abcde".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 2_(), c"".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 2_(), c"abcde".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 2_(), c"abcde".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 2_(), c"abcde".as_ptr(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 2_(), c"abcde".as_ptr(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 2_(), c"abcde".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 2_(), c"abcdefghij".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 2_(), c"abcdefghij".as_ptr(), 9_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 2_(), c"abcdefghij".as_ptr(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 3_(), c"".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 3_(), c"abcde".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 3_(), c"abcde".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 3_(), c"abcde".as_ptr(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 3_(), c"abcde".as_ptr(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 3_(), c"abcde".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 3_(), c"abcdefghij".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 3_(), c"abcdefghij".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 3_(), c"abcdefghij".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 3_(), c"abcdefghij".as_ptr(), 9_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 3_(), c"abcdefghij".as_ptr(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            3_(),
        )
    });
    return true;
}
pub unsafe fn test2_12() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 3_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 3_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 3_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            3_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 5_(), c"".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 5_(), c"abcde".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 5_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 5_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 5_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 5_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 5_(), c"abcdefghij".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 5_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 5_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 5_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 5_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 1_(), c"abcde".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 1_(), c"abcde".as_ptr(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 1_(), c"abcde".as_ptr(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 1_(), c"abcde".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 2_(), c"".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 2_(), c"abcde".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 2_(), c"abcde".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 2_(), c"abcde".as_ptr(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 2_(), c"abcde".as_ptr(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 2_(), c"abcde".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 2_(), c"abcdefghij".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 2_(), c"abcdefghij".as_ptr(), 9_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 2_(), c"abcdefghij".as_ptr(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 3_(), c"".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 3_(), c"abcde".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 3_(), c"abcde".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 3_(), c"abcde".as_ptr(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 3_(), c"abcde".as_ptr(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 3_(), c"abcde".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 3_(), c"abcdefghij".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 3_(), c"abcdefghij".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 3_(), c"abcdefghij".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 3_(), c"abcdefghij".as_ptr(), 9_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 3_(), c"abcdefghij".as_ptr(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            3_(),
        )
    });
    return true;
}
pub unsafe fn test3_13() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 4_(), c"".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 4_(), c"abcde".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 4_(), c"abcde".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 4_(), c"abcde".as_ptr(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 4_(), c"abcde".as_ptr(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 4_(), c"abcde".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 4_(), c"abcdefghij".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 4_(), c"abcdefghij".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 4_(), c"abcdefghij".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 4_(), c"abcdefghij".as_ptr(), 9_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 2_(), 4_(), c"abcdefghij".as_ptr(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            3_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            2_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 1_(), c"abcde".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 1_(), c"abcde".as_ptr(), 2_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 1_(), c"abcde".as_ptr(), 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 1_(), c"abcde".as_ptr(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 2_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 2_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 2_(), c"abcde".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 2_(), c"abcde".as_ptr(), 2_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 2_(), c"abcde".as_ptr(), 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 2_(), c"abcde".as_ptr(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 2_(), c"abcdefghij".as_ptr(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 2_(), c"abcdefghij".as_ptr(), 9_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 4_(), 2_(), c"abcdefghij".as_ptr(), 10_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            4_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    return true;
}
pub unsafe fn test3_14() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            3_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 4_(), c"".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 4_(), c"abcde".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 4_(), c"abcde".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 4_(), c"abcde".as_ptr(), 2_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 4_(), c"abcde".as_ptr(), 4_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 4_(), c"abcde".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 4_(), c"abcdefghij".as_ptr(), 0_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 4_(), c"abcdefghij".as_ptr(), 1_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 4_(), c"abcdefghij".as_ptr(), 5_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 4_(), c"abcdefghij".as_ptr(), 9_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 2_(), 4_(), c"abcdefghij".as_ptr(), 10_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            3_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            2_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 1_(), c"abcde".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 1_(), c"abcde".as_ptr(), 2_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 1_(), c"abcde".as_ptr(), 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 1_(), c"abcde".as_ptr(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 2_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 2_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 2_(), c"abcde".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 2_(), c"abcde".as_ptr(), 2_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 2_(), c"abcde".as_ptr(), 4_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 2_(), c"abcde".as_ptr(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 2_(), c"abcdefghij".as_ptr(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 2_(), c"abcdefghij".as_ptr(), 9_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 4_(), 2_(), c"abcdefghij".as_ptr(), 10_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            4_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    return true;
}
pub unsafe fn test4_15() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 6_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 6_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 6_(), 0_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 6_(), 0_(), c"abcde".as_ptr(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 6_(), 0_(), c"abcde".as_ptr(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 6_(), 0_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 6_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 6_(), 0_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 6_(), 0_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 6_(), 0_(), c"abcdefghij".as_ptr(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(&mut _s, 6_(), 0_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            6_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            6_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            6_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            6_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            6_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 2_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 4_(), -3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 5_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -18_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 2_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 9_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 10_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -14_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -15_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 9_(), c"".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 9_(), c"abcde".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 9_(), c"abcde".as_ptr(), 1_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 9_(), c"abcde".as_ptr(), 2_(), 7_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 9_(), c"abcde".as_ptr(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 9_(), c"abcde".as_ptr(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 9_(), c"abcdefghij".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 9_(), c"abcdefghij".as_ptr(), 1_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 9_(), c"abcdefghij".as_ptr(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 9_(), c"abcdefghij".as_ptr(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 9_(), c"abcdefghij".as_ptr(), 10_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            8_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -11_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 2_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 4_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 11_(), c"".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 11_(), c"abcde".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 11_(), c"abcde".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 11_(), c"abcde".as_ptr(), 2_(), 8_())
    });
    return true;
}
pub unsafe fn test4_16() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 6_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 6_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 6_(), 0_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 6_(), 0_(), c"abcde".as_ptr(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 6_(), 0_(), c"abcde".as_ptr(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 6_(), 0_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 6_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 6_(), 0_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 6_(), 0_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 6_(), 0_(), c"abcdefghij".as_ptr(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(&mut _s, 6_(), 0_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            6_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            6_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            6_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            6_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            6_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 2_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 4_(), -3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 5_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -18_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 2_(), 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 1_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 9_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 5_(), c"abcdefghij".as_ptr(), 10_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -14_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -15_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 9_(), c"".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 9_(), c"abcde".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 9_(), c"abcde".as_ptr(), 1_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 9_(), c"abcde".as_ptr(), 2_(), 7_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 9_(), c"abcde".as_ptr(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 9_(), c"abcde".as_ptr(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 9_(), c"abcdefghij".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 9_(), c"abcdefghij".as_ptr(), 1_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 9_(), c"abcdefghij".as_ptr(), 5_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 9_(), c"abcdefghij".as_ptr(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 9_(), c"abcdefghij".as_ptr(), 10_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            8_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -11_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 2_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 4_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 11_(), c"".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 11_(), c"abcde".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 11_(), c"abcde".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 11_(), c"abcde".as_ptr(), 2_(), 8_())
    });
    return true;
}
pub unsafe fn test5_17() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 11_(), c"abcde".as_ptr(), 4_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 11_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 11_(), c"abcdefghij".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 11_(), c"abcdefghij".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 11_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 11_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 11_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 8_(), c"".as_ptr(), 0_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 8_(), c"abcde".as_ptr(), 0_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 8_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 8_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 8_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 8_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 8_(), c"abcdefghij".as_ptr(), 0_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 8_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 8_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 8_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 8_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            8_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            8_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            8_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            8_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            8_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            8_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 10_(), c"".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 10_(), c"abcde".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 10_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 10_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 10_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 10_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 10_(), c"abcdefghij".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 10_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    return true;
}
pub unsafe fn test5_18() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 11_(), c"abcde".as_ptr(), 4_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 11_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 11_(), c"abcdefghij".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 11_(), c"abcdefghij".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 11_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 11_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 11_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 4_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 8_(), c"".as_ptr(), 0_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 8_(), c"abcde".as_ptr(), 0_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 8_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 8_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 8_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 8_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 8_(), c"abcdefghij".as_ptr(), 0_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 8_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 8_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 8_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 8_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            8_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            8_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            8_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            8_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            8_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            8_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 10_(), c"".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 10_(), c"abcde".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 10_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 10_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 10_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 10_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 10_(), c"abcdefghij".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 10_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    return true;
}
pub unsafe fn test6_19() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 10_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 10_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 10_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 2_(), c"".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 2_(), c"abcde".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 2_(), c"abcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 2_(), c"abcde".as_ptr(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 2_(), c"abcde".as_ptr(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 2_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 2_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 2_(), c"abcdefghij".as_ptr(), 9_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 2_(), c"abcdefghij".as_ptr(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 4_(), c"".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 4_(), c"abcde".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 4_(), c"abcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 4_(), c"abcde".as_ptr(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 4_(), c"abcde".as_ptr(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 4_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 4_(), c"abcdefghij".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 4_(), c"abcdefghij".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 4_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 4_(), c"abcdefghij".as_ptr(), 9_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 4_(), c"abcdefghij".as_ptr(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 5_(), c"".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 5_(), c"abcde".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 5_(), c"abcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 5_(), c"abcde".as_ptr(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 5_(), c"abcde".as_ptr(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 5_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 5_(), c"abcdefghij".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 5_(), c"abcdefghij".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 5_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 5_(), c"abcdefghij".as_ptr(), 9_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 5_(), c"abcdefghij".as_ptr(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 6_(), c"".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 6_(), c"abcde".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 6_(), c"abcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 6_(), c"abcde".as_ptr(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 6_(), c"abcde".as_ptr(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 6_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 6_(), c"abcdefghij".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 6_(), c"abcdefghij".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 6_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 6_(), c"abcdefghij".as_ptr(), 9_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 5_(), 6_(), c"abcdefghij".as_ptr(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            5_(),
        )
    });
    return true;
}
pub unsafe fn test6_20() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 10_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 10_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 10_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 2_(), c"".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 2_(), c"abcde".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 2_(), c"abcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 2_(), c"abcde".as_ptr(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 2_(), c"abcde".as_ptr(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 2_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 2_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 2_(), c"abcdefghij".as_ptr(), 9_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 2_(), c"abcdefghij".as_ptr(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            2_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 4_(), c"".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 4_(), c"abcde".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 4_(), c"abcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 4_(), c"abcde".as_ptr(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 4_(), c"abcde".as_ptr(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 4_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 4_(), c"abcdefghij".as_ptr(), 0_(), 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 4_(), c"abcdefghij".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 4_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 4_(), c"abcdefghij".as_ptr(), 9_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 4_(), c"abcdefghij".as_ptr(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            4_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            4_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 5_(), c"".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 5_(), c"abcde".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 5_(), c"abcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 5_(), c"abcde".as_ptr(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 5_(), c"abcde".as_ptr(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 5_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 5_(), c"abcdefghij".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 5_(), c"abcdefghij".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 5_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 5_(), c"abcdefghij".as_ptr(), 9_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 5_(), c"abcdefghij".as_ptr(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 6_(), c"".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 6_(), c"abcde".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 6_(), c"abcde".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 6_(), c"abcde".as_ptr(), 2_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 6_(), c"abcde".as_ptr(), 4_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 6_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 6_(), c"abcdefghij".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 6_(), c"abcdefghij".as_ptr(), 1_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 6_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 6_(), c"abcdefghij".as_ptr(), 9_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 5_(), 6_(), c"abcdefghij".as_ptr(), 10_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            5_(),
        )
    });
    return true;
}
pub unsafe fn test7_21() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            5_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 1_(), c"abcde".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 1_(), c"abcde".as_ptr(), 2_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 1_(), c"abcde".as_ptr(), 4_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 1_(), c"abcde".as_ptr(), 5_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 2_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 2_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 2_(), c"abcde".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 2_(), c"abcde".as_ptr(), 2_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 2_(), c"abcde".as_ptr(), 4_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 2_(), c"abcde".as_ptr(), 5_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 2_(), c"abcdefghij".as_ptr(), 5_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 2_(), c"abcdefghij".as_ptr(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 9_(), 2_(), c"abcdefghij".as_ptr(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            9_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 11_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 11_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 11_(), 0_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 11_(), 0_(), c"abcde".as_ptr(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 11_(), 0_(), c"abcde".as_ptr(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 11_(), 0_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 11_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 11_(), 0_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 11_(), 0_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 11_(), 0_(), c"abcdefghij".as_ptr(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(&mut _s, 11_(), 0_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            11_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            11_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            11_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            11_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            11_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
        )
    });
    return true;
}
pub unsafe fn test7_22() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            5_(),
            6_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 1_(), c"abcde".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 1_(), c"abcde".as_ptr(), 2_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 1_(), c"abcde".as_ptr(), 4_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 1_(), c"abcde".as_ptr(), 5_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 2_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 2_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 2_(), c"abcde".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 2_(), c"abcde".as_ptr(), 2_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 2_(), c"abcde".as_ptr(), 4_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 2_(), c"abcde".as_ptr(), 5_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 2_(), c"abcdefghij".as_ptr(), 5_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 2_(), c"abcdefghij".as_ptr(), 9_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 9_(), 2_(), c"abcdefghij".as_ptr(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            9_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 11_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 11_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 11_(), 0_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 11_(), 0_(), c"abcde".as_ptr(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 11_(), 0_(), c"abcde".as_ptr(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 11_(), 0_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 11_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 11_(), 0_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 11_(), 0_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 11_(), 0_(), c"abcdefghij".as_ptr(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(&mut _s, 11_(), 0_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            11_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            11_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            11_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            11_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            11_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
        )
    });
    return true;
}
pub unsafe fn test8_23() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 2_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 4_(), -3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 5_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -18_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 2_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 4_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 19_(), c"".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 19_(), c"abcde".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 19_(), c"abcde".as_ptr(), 1_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 19_(), c"abcde".as_ptr(), 2_(), 17_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 19_(), c"abcde".as_ptr(), 4_(), 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 19_(), c"abcde".as_ptr(), 5_(), 14_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 19_(), c"abcdefghij".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 19_(), c"abcdefghij".as_ptr(), 1_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 19_(), c"abcdefghij".as_ptr(), 5_(), 14_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 19_(), c"abcdefghij".as_ptr(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 19_(), c"abcdefghij".as_ptr(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            18_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 20_(), c"".as_ptr(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 20_(), c"abcde".as_ptr(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 20_(), c"abcde".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 20_(), c"abcde".as_ptr(), 2_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 20_(), c"abcde".as_ptr(), 4_(), 16_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 20_(), c"abcde".as_ptr(), 5_(), 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 20_(), c"abcdefghij".as_ptr(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 20_(), c"abcdefghij".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 20_(), c"abcdefghij".as_ptr(), 5_(), 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 20_(), c"abcdefghij".as_ptr(), 9_(), 11_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 20_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 21_(), c"".as_ptr(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 21_(), c"abcde".as_ptr(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 21_(), c"abcde".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 21_(), c"abcde".as_ptr(), 2_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 21_(), c"abcde".as_ptr(), 4_(), 16_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 21_(), c"abcde".as_ptr(), 5_(), 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 21_(), c"abcdefghij".as_ptr(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 21_(), c"abcdefghij".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 21_(), c"abcdefghij".as_ptr(), 5_(), 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 21_(), c"abcdefghij".as_ptr(), 9_(), 11_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 0_(), 21_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            21_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            21_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            21_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            21_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            0_(),
            21_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    return true;
}
pub unsafe fn test8_24() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 2_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 4_(), -3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcde".as_ptr(), 5_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -18_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 2_(), 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 4_(), 6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcde".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 1_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 5_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 10_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 19_(), c"".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 19_(), c"abcde".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 19_(), c"abcde".as_ptr(), 1_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 19_(), c"abcde".as_ptr(), 2_(), 17_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 19_(), c"abcde".as_ptr(), 4_(), 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 19_(), c"abcde".as_ptr(), 5_(), 14_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 19_(), c"abcdefghij".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 19_(), c"abcdefghij".as_ptr(), 1_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 19_(), c"abcdefghij".as_ptr(), 5_(), 14_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 19_(), c"abcdefghij".as_ptr(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 19_(), c"abcdefghij".as_ptr(), 10_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            18_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 20_(), c"".as_ptr(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 20_(), c"abcde".as_ptr(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 20_(), c"abcde".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 20_(), c"abcde".as_ptr(), 2_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 20_(), c"abcde".as_ptr(), 4_(), 16_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 20_(), c"abcde".as_ptr(), 5_(), 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 20_(), c"abcdefghij".as_ptr(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 20_(), c"abcdefghij".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 20_(), c"abcdefghij".as_ptr(), 5_(), 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 20_(), c"abcdefghij".as_ptr(), 9_(), 11_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 20_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 21_(), c"".as_ptr(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 21_(), c"abcde".as_ptr(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 21_(), c"abcde".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 21_(), c"abcde".as_ptr(), 2_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 21_(), c"abcde".as_ptr(), 4_(), 16_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 21_(), c"abcde".as_ptr(), 5_(), 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 21_(), c"abcdefghij".as_ptr(), 0_(), 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 21_(), c"abcdefghij".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 21_(), c"abcdefghij".as_ptr(), 5_(), 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 21_(), c"abcdefghij".as_ptr(), 9_(), 11_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 0_(), 21_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            21_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            21_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            21_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            21_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            0_(),
            21_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    return true;
}
pub unsafe fn test9_25() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 18_(), c"".as_ptr(), 0_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 18_(), c"abcde".as_ptr(), 0_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 18_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 18_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 18_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 18_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 18_(), c"abcdefghij".as_ptr(), 0_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 18_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 18_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 18_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 18_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            18_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            18_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            18_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            18_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            18_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            18_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 19_(), c"".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 19_(), c"abcde".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 19_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 19_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 19_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 19_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 19_(), c"abcdefghij".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 19_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 19_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 19_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 19_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 20_(), c"".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 20_(), c"abcde".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 20_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 20_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 20_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 20_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 20_(), c"abcdefghij".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 20_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 20_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 20_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 1_(), 20_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            1_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    return true;
}
pub unsafe fn test9_26() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 9_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 18_(), c"".as_ptr(), 0_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 18_(), c"abcde".as_ptr(), 0_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 18_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 18_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 18_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 18_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 18_(), c"abcdefghij".as_ptr(), 0_(), 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 18_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 18_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 18_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 18_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            18_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            18_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            18_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            18_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            18_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            18_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 19_(), c"".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 19_(), c"abcde".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 19_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 19_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 19_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 19_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 19_(), c"abcdefghij".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 19_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 19_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 19_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 19_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            19_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 20_(), c"".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 20_(), c"abcde".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 20_(), c"abcde".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 20_(), c"abcde".as_ptr(), 2_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 20_(), c"abcde".as_ptr(), 4_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 20_(), c"abcde".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 20_(), c"abcdefghij".as_ptr(), 0_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 20_(), c"abcdefghij".as_ptr(), 1_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 20_(), c"abcdefghij".as_ptr(), 5_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 20_(), c"abcdefghij".as_ptr(), 9_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 1_(), 20_(), c"abcdefghij".as_ptr(), 10_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            1_(),
            20_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    return true;
}
pub unsafe fn test10_27() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 5_(), c"".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 5_(), c"abcde".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 5_(), c"abcde".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 5_(), c"abcde".as_ptr(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 5_(), c"abcde".as_ptr(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 5_(), c"abcde".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 5_(), c"abcdefghij".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 5_(), c"abcdefghij".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 5_(), c"abcdefghij".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 5_(), c"abcdefghij".as_ptr(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 5_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 9_(), c"".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 9_(), c"abcde".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 9_(), c"abcde".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 9_(), c"abcde".as_ptr(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 9_(), c"abcde".as_ptr(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 9_(), c"abcde".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 9_(), c"abcdefghij".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 9_(), c"abcdefghij".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 9_(), c"abcdefghij".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 9_(), c"abcdefghij".as_ptr(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 9_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 10_(), c"".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 10_(), c"abcde".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 10_(), c"abcde".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 10_(), c"abcde".as_ptr(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 10_(), c"abcde".as_ptr(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 10_(), c"abcde".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 10_(), c"abcdefghij".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 10_(), c"abcdefghij".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 10_(), c"abcdefghij".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 10_(), c"abcdefghij".as_ptr(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 10_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 11_(), c"".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 11_(), c"abcde".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 11_(), c"abcde".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 11_(), c"abcde".as_ptr(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 11_(), c"abcde".as_ptr(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 11_(), c"abcde".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 11_(), c"abcdefghij".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 11_(), c"abcdefghij".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 11_(), c"abcdefghij".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 11_(), c"abcdefghij".as_ptr(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 10_(), 11_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            10_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    return true;
}
pub unsafe fn test10_28() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcde".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 5_(), c"".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 5_(), c"abcde".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 5_(), c"abcde".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 5_(), c"abcde".as_ptr(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 5_(), c"abcde".as_ptr(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 5_(), c"abcde".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 5_(), c"abcdefghij".as_ptr(), 0_(), 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 5_(), c"abcdefghij".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 5_(), c"abcdefghij".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 5_(), c"abcdefghij".as_ptr(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 5_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            5_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            5_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 9_(), c"".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 9_(), c"abcde".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 9_(), c"abcde".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 9_(), c"abcde".as_ptr(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 9_(), c"abcde".as_ptr(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 9_(), c"abcde".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 9_(), c"abcdefghij".as_ptr(), 0_(), 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 9_(), c"abcdefghij".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 9_(), c"abcdefghij".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 9_(), c"abcdefghij".as_ptr(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 9_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            9_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            9_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 10_(), c"".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 10_(), c"abcde".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 10_(), c"abcde".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 10_(), c"abcde".as_ptr(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 10_(), c"abcde".as_ptr(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 10_(), c"abcde".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 10_(), c"abcdefghij".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 10_(), c"abcdefghij".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 10_(), c"abcdefghij".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 10_(), c"abcdefghij".as_ptr(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 10_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            10_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 11_(), c"".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 11_(), c"abcde".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 11_(), c"abcde".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 11_(), c"abcde".as_ptr(), 2_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 11_(), c"abcde".as_ptr(), 4_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 11_(), c"abcde".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 11_(), c"abcdefghij".as_ptr(), 0_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 11_(), c"abcdefghij".as_ptr(), 1_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 11_(), c"abcdefghij".as_ptr(), 5_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 11_(), c"abcdefghij".as_ptr(), 9_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 10_(), 11_(), c"abcdefghij".as_ptr(), 10_(), 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            10_(),
            11_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    return true;
}
pub unsafe fn test11_29() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 1_(), c"abcde".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 1_(), c"abcde".as_ptr(), 2_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 1_(), c"abcde".as_ptr(), 4_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 1_(), c"abcde".as_ptr(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 2_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 2_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 2_(), c"abcde".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 2_(), c"abcde".as_ptr(), 2_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 2_(), c"abcde".as_ptr(), 4_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 2_(), c"abcde".as_ptr(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 2_(), c"abcdefghij".as_ptr(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 2_(), c"abcdefghij".as_ptr(), 9_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 19_(), 2_(), c"abcdefghij".as_ptr(), 10_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            19_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            20_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            20_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            20_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            20_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            20_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 1_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 1_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 1_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 1_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 1_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 1_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 1_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 20_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            20_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            20_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            20_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            20_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            20_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 21_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 21_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 21_(), 0_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 21_(), 0_(), c"abcde".as_ptr(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 21_(), 0_(), c"abcde".as_ptr(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 21_(), 0_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 21_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 21_(), 0_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 21_(), 0_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 21_(), 0_(), c"abcdefghij".as_ptr(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(&mut _s, 21_(), 0_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            21_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            21_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            21_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            21_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_5(
            &mut _s,
            21_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
        )
    });
    return true;
}
pub unsafe fn test11_30() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 1_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 1_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 1_(), c"abcde".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 1_(), c"abcde".as_ptr(), 2_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 1_(), c"abcde".as_ptr(), 4_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 1_(), c"abcde".as_ptr(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 1_(), c"abcdefghij".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 1_(), c"abcdefghij".as_ptr(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 1_(), c"abcdefghij".as_ptr(), 9_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 1_(), c"abcdefghij".as_ptr(), 10_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 2_(), c"".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 2_(), c"abcde".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 2_(), c"abcde".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 2_(), c"abcde".as_ptr(), 2_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 2_(), c"abcde".as_ptr(), 4_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 2_(), c"abcde".as_ptr(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 2_(), c"abcdefghij".as_ptr(), 0_(), 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 2_(), c"abcdefghij".as_ptr(), 1_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 2_(), c"abcdefghij".as_ptr(), 5_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 2_(), c"abcdefghij".as_ptr(), 9_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 19_(), 2_(), c"abcdefghij".as_ptr(), 10_(), 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            19_(),
            2_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 0_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 0_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 0_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 0_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 0_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 0_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 0_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 0_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            20_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            20_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            20_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            20_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            20_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 1_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 1_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 1_(), c"abcde".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 1_(), c"abcde".as_ptr(), 2_(), -2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 1_(), c"abcde".as_ptr(), 4_(), -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 1_(), c"abcde".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 1_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 1_(), c"abcdefghij".as_ptr(), 1_(), -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 1_(), c"abcdefghij".as_ptr(), 5_(), -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 1_(), c"abcdefghij".as_ptr(), 9_(), -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 20_(), 1_(), c"abcdefghij".as_ptr(), 10_(), -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            20_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            20_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            -1_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            20_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            -10_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            20_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            -19_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            20_(),
            1_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            -20_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 21_(), 0_(), c"".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 21_(), 0_(), c"abcde".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 21_(), 0_(), c"abcde".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 21_(), 0_(), c"abcde".as_ptr(), 2_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 21_(), 0_(), c"abcde".as_ptr(), 4_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 21_(), 0_(), c"abcde".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 21_(), 0_(), c"abcdefghij".as_ptr(), 0_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 21_(), 0_(), c"abcdefghij".as_ptr(), 1_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 21_(), 0_(), c"abcdefghij".as_ptr(), 5_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 21_(), 0_(), c"abcdefghij".as_ptr(), 9_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(&mut _s, 21_(), 0_(), c"abcdefghij".as_ptr(), 10_(), 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            21_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            0_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            21_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            1_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            21_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            10_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            21_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            19_(),
            0_(),
        )
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        test_6(
            &mut _s,
            21_(),
            0_(),
            c"abcdefghijklmnopqrst".as_ptr(),
            20_(),
            0_(),
        )
    });
    return true;
}
pub unsafe fn test_31() {
    (unsafe { test0_7() });
    (unsafe { test1_9() });
    (unsafe { test2_11() });
    (unsafe { test3_13() });
    (unsafe { test4_15() });
    (unsafe { test5_17() });
    (unsafe { test6_19() });
    (unsafe { test7_21() });
    (unsafe { test8_23() });
    (unsafe { test9_25() });
    (unsafe { test10_27() });
    (unsafe { test11_29() });
    const _: () = assert!(true, "test0<S>()");;
    const _: () = assert!(true, "test1<S>()");;
    const _: () = assert!(true, "test2<S>()");;
    const _: () = assert!(true, "test3<S>()");;
    const _: () = assert!(true, "test4<S>()");;
    const _: () = assert!(true, "test5<S>()");;
    const _: () = assert!(true, "test6<S>()");;
    const _: () = assert!(true, "test7<S>()");;
    const _: () = assert!(true, "test8<S>()");;
    const _: () = assert!(true, "test9<S>()");;
    const _: () = assert!(true, "test10<S>()");;
    const _: () = assert!(true, "test11<S>()");;
}
pub unsafe fn test_32() {
    (unsafe { test0_8() });
    (unsafe { test1_10() });
    (unsafe { test2_12() });
    (unsafe { test3_14() });
    (unsafe { test4_16() });
    (unsafe { test5_18() });
    (unsafe { test6_20() });
    (unsafe { test7_22() });
    (unsafe { test8_24() });
    (unsafe { test9_26() });
    (unsafe { test10_28() });
    (unsafe { test11_30() });
    const _: () = assert!(true, "test0<S>()");;
    const _: () = assert!(true, "test1<S>()");;
    const _: () = assert!(true, "test2<S>()");;
    const _: () = assert!(true, "test3<S>()");;
    const _: () = assert!(true, "test4<S>()");;
    const _: () = assert!(true, "test5<S>()");;
    const _: () = assert!(true, "test6<S>()");;
    const _: () = assert!(true, "test7<S>()");;
    const _: () = assert!(true, "test8<S>()");;
    const _: () = assert!(true, "test9<S>()");;
    const _: () = assert!(true, "test10<S>()");;
    const _: () = assert!(true, "test11<S>()");;
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
    (unsafe { test_31() });
    (unsafe { test_32() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
