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
                memset_5(
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
                    memset_5(
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
pub unsafe fn is_string_asan_correct_6(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_7(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_8(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn test_9(mut s: (), mut pos: (), mut str: *const (), mut n: (), mut expected: ()) {
    let old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if ((pos) <= (old_size)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_10(mut s: (), mut pos: (), mut str: *const (), mut n: (), mut expected: ()) {
    let old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if ((pos) <= (old_size)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_11(mut s: (), mut pos: (), mut str: *const (), mut n: (), mut expected: ()) {
    let old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if ((pos) <= (old_size)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_12() -> () {
    {
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234567890123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345678901234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"123456789abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234567890abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234567890abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234567890123456789abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345678901234567890abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a12bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1234bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a12345bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a12345bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a123456789bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1234567890bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1234567890bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1234567890123456789bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a12345678901234567890bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ab1cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ab12cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ab1234cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ab12345cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ab1cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ab12345cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ab123456789cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ab1234567890cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ab1cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ab1234567890cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ab1234567890123456789cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"ab12345678901234567890cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcd1e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcd12e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcd1234e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcd12345e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcd1e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcd12345e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcd123456789e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcd1234567890e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcd1e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcd1234567890e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcd1234567890123456789e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcd12345678901234567890e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde12".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1234".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1234567890123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde12345678901234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"123456789abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234567890abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234567890abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234567890123456789abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345678901234567890abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a12bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1234bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a12345bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a12345bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a123456789bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1234567890bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1234567890bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1234567890123456789bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a12345678901234567890bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde12fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1234fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde12345fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde12345fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde123456789fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1234567890fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1234567890fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde1234567890123456789fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcde12345678901234567890fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghi1j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghi12j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghi1234j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghi12345j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghi1j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghi12345j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghi123456789j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghi1234567890j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghi1j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghi1234567890j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghi1234567890123456789j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghi12345678901234567890j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij12".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1234".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1234567890123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij12345678901234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"123456789abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234567890abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234567890abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"1234567890123456789abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"12345678901234567890abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a12bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1234bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a12345bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a12345bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a123456789bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1234567890bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1234567890bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a1234567890123456789bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"a12345678901234567890bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij12klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1234klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij12345klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij12345klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij123456789klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1234567890klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1234567890klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij1234567890123456789klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghij12345678901234567890klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs12t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1234t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs12345t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs12345t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs123456789t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1234567890t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1234567890t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1234567890123456789t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs12345678901234567890t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst12".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1234".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1234567890123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst12345678901234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_9(
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
    }
    {
        let mut s_short: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"123/".as_ptr() },
                None,
            );
        let mut s_long: () =
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"Lorem ipsum dolor sit amet, consectetur/".as_ptr() },
                None,
            );
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
pub unsafe fn test_13() -> () {
    {
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234567890123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12345678901234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12345abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12345abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"123456789abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234567890abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234567890abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234567890123456789abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12345678901234567890abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a12bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1234bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a12345bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a12345bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a123456789bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1234567890bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1234567890bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1234567890123456789bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a12345678901234567890bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"ab1cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"ab12cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"ab1234cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"ab12345cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"ab1cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"ab12345cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"ab123456789cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"ab1234567890cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"ab1cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"ab1234567890cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"ab1234567890123456789cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"ab12345678901234567890cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcd1e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcd12e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcd1234e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcd12345e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcd1e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcd12345e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcd123456789e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcd1234567890e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcd1e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcd1234567890e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcd1234567890123456789e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcd12345678901234567890e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde12".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1234".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1234567890123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde12345678901234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12345abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12345abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"123456789abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234567890abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234567890abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234567890123456789abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12345678901234567890abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a12bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1234bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a12345bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a12345bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a123456789bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1234567890bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1234567890bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1234567890123456789bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a12345678901234567890bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde12fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1234fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde12345fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde12345fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde123456789fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1234567890fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1234567890fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde1234567890123456789fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcde12345678901234567890fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghi1j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghi12j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghi1234j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghi12345j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghi1j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghi12345j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghi123456789j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghi1234567890j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghi1j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghi1234567890j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghi1234567890123456789j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghi12345678901234567890j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij12".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1234".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1234567890123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij12345678901234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12345abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12345abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"123456789abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234567890abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234567890abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"1234567890123456789abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"12345678901234567890abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a12bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1234bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a12345bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a12345bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a123456789bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1234567890bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1234567890bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a1234567890123456789bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"a12345678901234567890bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij12klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1234klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij12345klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij12345klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij123456789klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1234567890klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1234567890klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij1234567890123456789klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghij12345678901234567890klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs12t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1234t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs12345t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs12345t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs123456789t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1234567890t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1234567890t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1234567890123456789t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs12345678901234567890t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst12".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1234".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1234567890123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst12345678901234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_10(
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
    }
    {
        let mut s_short: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"123/".as_ptr() },
                None,
            );
        let mut s_long: () =
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"Lorem ipsum dolor sit amet, consectetur/".as_ptr() },
                None,
            );
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
pub unsafe fn test_14() -> () {
    {
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234567890123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12345678901234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12345abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12345abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"123456789abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234567890abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234567890abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234567890123456789abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12345678901234567890abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a12bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1234bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a12345bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a12345bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a123456789bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1234567890bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1234567890bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1234567890123456789bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a12345678901234567890bcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"ab1cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"ab12cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"ab1234cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"ab12345cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"ab1cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"ab12345cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"ab123456789cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"ab1234567890cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"ab1cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"ab1234567890cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"ab1234567890123456789cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                2_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"ab12345678901234567890cde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcd1e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcd12e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcd1234e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcd12345e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcd1e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcd12345e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcd123456789e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcd1234567890e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcd1e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcd1234567890e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcd1234567890123456789e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                4_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcd12345678901234567890e".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde12".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1234".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1234567890123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde12345678901234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde".as_ptr() },
                    None,
                ),
                6_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12345abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12345abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"123456789abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234567890abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234567890abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234567890123456789abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12345678901234567890abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a12bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1234bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a12345bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a12345bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a123456789bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1234567890bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1234567890bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1234567890123456789bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a12345678901234567890bcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde12fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1234fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde12345fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde12345fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde123456789fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1234567890fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1234567890fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde1234567890123456789fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                5_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcde12345678901234567890fghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghi1j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghi12j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghi1234j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghi12345j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghi1j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghi12345j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghi123456789j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghi1234567890j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghi1j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghi1234567890j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghi1234567890123456789j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                9_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghi12345678901234567890j".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij12".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1234".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1234567890123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij12345678901234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij".as_ptr() },
                    None,
                ),
                11_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12345abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12345abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"123456789abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234567890abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234567890abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"1234567890123456789abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                0_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"12345678901234567890abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a12bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1234bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a12345bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a12345bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a123456789bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1234567890bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1234567890bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a1234567890123456789bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                1_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"a12345678901234567890bcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij12klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1234klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij12345klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij12345klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij123456789klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1234567890klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1234567890klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij1234567890123456789klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                10_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghij12345678901234567890klmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs12t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1234t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs12345t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs12345t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs123456789t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1234567890t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1234567890t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs1234567890123456789t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                19_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrs12345678901234567890t".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst12".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1234".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst12345".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst1234567890123456789".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                20_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst12345678901234567890".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                2_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                4_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                5_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                9_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"1234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                0_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                1_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                10_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                19_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
        (unsafe {
            test_11(
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"abcdefghijklmnopqrst".as_ptr() },
                    None,
                ),
                21_(),
                c"12345678901234567890".as_ptr(),
                20_(),
                std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                    { c"can\'t happen".as_ptr() },
                    None,
                ),
            )
        });
    }
    {
        let mut s_short: () =
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"123/".as_ptr() },
                None,
            );
        let mut s_long: () =
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"Lorem ipsum dolor sit amet, consectetur/".as_ptr() },
                None,
            );
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
    (unsafe { test_12() });
    (unsafe { test_13() });
    (unsafe { test_14() });
    const _: () = assert!(true, "test<std::string>()");;
    const _: () = assert!(
        true,
        "test<std::basic_string<char, std::char_traits<char>, min_allocator<char>>>()"
    );;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
