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
pub unsafe fn test_5(s: *const (), mut pos1: (), mut n1: (), str: *const (), mut x: ()) {
    if ((pos1) <= (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test_6(s: *const (), mut pos1: (), mut n1: (), str: *const (), mut x: ()) {
    if ((pos1) <= (unimplemented!())) {
        unimplemented!();
    }
}
pub unsafe fn test0_7() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, -19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 2_(), &mut _str, -3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 2_(), &mut _str, -8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 2_(), &mut _str, -18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 4_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 4_(), &mut _str, -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 4_(), &mut _str, -6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 4_(), &mut _str, -16_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 5_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 5_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 5_(), &mut _str, -15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 6_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 6_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 6_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 6_(), &mut _str, -15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 2_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 2_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 2_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 3_(), &mut _str, 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 3_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 3_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 3_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 4_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 4_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 4_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 4_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 5_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 5_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 5_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 5_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 1_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 1_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 1_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 3_(), &mut _str, 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 3_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 3_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 3_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 4_(), &mut _str, 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 4_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 4_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 2_(), 4_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 4_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 4_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 4_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 4_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 4_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 4_(), 1_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 4_(), 1_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 4_(), 1_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 4_(), 2_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 4_(), 2_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 4_(), 2_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 4_(), 2_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 1_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 1_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 1_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 1_(), &mut _str, -20_())
    });
    return true;
}
pub unsafe fn test0_8() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, -19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 2_(), &mut _str, -3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 2_(), &mut _str, -8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 2_(), &mut _str, -18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 4_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 4_(), &mut _str, -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 4_(), &mut _str, -6_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 4_(), &mut _str, -16_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 5_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 5_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 5_(), &mut _str, -15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 6_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 6_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 6_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 6_(), &mut _str, -15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 2_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 2_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 2_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 3_(), &mut _str, 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 3_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 3_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 3_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 4_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 4_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 4_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 4_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 5_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 5_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 5_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 5_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 1_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 1_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 1_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 3_(), &mut _str, 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 3_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 3_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 3_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 4_(), &mut _str, 3_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 4_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 4_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 2_(), 4_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 4_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 4_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 4_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 4_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 4_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 4_(), 1_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 4_(), 1_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 4_(), 1_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 4_(), 2_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 4_(), 2_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 4_(), 2_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 4_(), 2_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 1_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 1_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 1_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 1_(), &mut _str, -20_())
    });
    return true;
}
pub unsafe fn test1_9() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 6_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 6_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 6_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 6_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, -19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 5_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 5_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 5_(), &mut _str, -15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 9_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 9_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 9_(), &mut _str, -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 9_(), &mut _str, -11_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 10_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 10_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 10_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 10_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 11_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 11_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 11_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 11_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 4_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 4_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 4_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 4_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 8_(), &mut _str, 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 8_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 8_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 8_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 9_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 9_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 9_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 9_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 10_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 10_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 10_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 10_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 1_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 1_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 1_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 2_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 2_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 2_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 4_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 4_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 4_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 4_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 6_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 6_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 6_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 5_(), 6_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 9_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 9_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 9_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 9_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 9_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 9_(), 1_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 9_(), 1_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 9_(), 1_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 9_(), 2_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 9_(), 2_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 9_(), 2_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 9_(), 2_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 1_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 1_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 1_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 1_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 11_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 11_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 11_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 11_(), 0_(), &mut _str, 0_())
    });
    return true;
}
pub unsafe fn test1_10() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 6_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 6_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 6_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcde".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 6_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, -19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 5_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 5_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 5_(), &mut _str, -15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 9_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 9_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 9_(), &mut _str, -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 9_(), &mut _str, -11_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 10_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 10_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 10_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 10_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 11_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 11_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 11_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 11_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 4_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 4_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 4_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 4_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 8_(), &mut _str, 8_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 8_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 8_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 8_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 9_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 9_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 9_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 9_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 10_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 10_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 10_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 10_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 1_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 1_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 1_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 2_(), &mut _str, 2_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 2_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 2_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 2_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 4_(), &mut _str, 4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 4_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 4_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 4_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 6_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 6_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 6_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 5_(), 6_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 9_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 9_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 9_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 9_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 9_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 9_(), 1_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 9_(), 1_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 9_(), 1_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 9_(), 2_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 9_(), 2_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 9_(), 2_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 9_(), 2_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 1_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 1_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 1_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 1_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 11_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 11_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 11_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghij".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 11_(), 0_(), &mut _str, 0_())
    });
    return true;
}
pub unsafe fn test2_11() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 1_(), &mut _str, -19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 10_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 10_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 10_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 10_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 19_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 19_(), &mut _str, 14_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 19_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 19_(), &mut _str, -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 20_(), &mut _str, 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 20_(), &mut _str, 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 20_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 20_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 21_(), &mut _str, 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 21_(), &mut _str, 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 21_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 0_(), 21_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 9_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 9_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 9_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 9_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 18_(), &mut _str, 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 18_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 18_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 18_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 19_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 19_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 19_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 19_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 20_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 20_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 20_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 1_(), 20_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 1_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 1_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 1_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 5_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 5_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 5_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 9_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 9_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 9_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 9_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 10_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 10_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 10_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 10_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 11_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 11_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 11_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 10_(), 11_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 19_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 19_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 19_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 19_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 19_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 19_(), 1_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 19_(), 1_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 19_(), 1_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 19_(), 2_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 19_(), 2_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 19_(), 2_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 19_(), 2_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 20_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 20_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 20_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 20_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 20_(), 1_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 20_(), 1_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 20_(), 1_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 20_(), 1_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_5(&mut _s, 21_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_5(&mut _s, 21_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_5(&mut _s, 21_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_5(&mut _s, 21_(), 0_(), &mut _str, 0_())
    });
    return true;
}
pub unsafe fn test2_12() -> () {
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, -4_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, -9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 1_(), &mut _str, -19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 10_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 10_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 10_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 10_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 19_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 19_(), &mut _str, 14_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 19_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 19_(), &mut _str, -1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 20_(), &mut _str, 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 20_(), &mut _str, 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 20_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 20_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 21_(), &mut _str, 20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 21_(), &mut _str, 15_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 21_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 0_(), 21_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 9_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 9_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 9_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 9_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 18_(), &mut _str, 18_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 18_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 18_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 18_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 19_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 19_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 19_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 19_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 20_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 20_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 20_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 1_(), 20_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 1_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 1_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 1_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 5_(), &mut _str, 5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 5_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 5_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 5_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 9_(), &mut _str, 9_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 9_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 9_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 9_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 10_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 10_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 10_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 10_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 11_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 11_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 11_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 10_(), 11_(), &mut _str, 10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 19_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 19_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 19_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 19_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 19_(), 1_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 19_(), 1_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 19_(), 1_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 19_(), 1_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 19_(), 2_(), &mut _str, 1_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 19_(), 2_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 19_(), 2_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 19_(), 2_(), &mut _str, 19_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 20_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 20_(), 0_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 20_(), 0_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 20_(), 0_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 20_(), 1_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 20_(), 1_(), &mut _str, -5_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 20_(), 1_(), &mut _str, -10_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 20_(), 1_(), &mut _str, -20_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            );
        test_6(&mut _s, 21_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            );
        test_6(&mut _s, 21_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            );
        test_6(&mut _s, 21_(), 0_(), &mut _str, 0_())
    });
    (unsafe {
        let mut _s: () = std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
            { c"abcdefghijklmnopqrst".as_ptr() },
            None,
        );
        let mut _str: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            );
        test_6(&mut _s, 21_(), 0_(), &mut _str, 0_())
    });
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
    {
        let mut s: () = std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
            { c" !".as_ptr() },
            None,
        );
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
