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
pub unsafe fn test_9(mut s: (), mut n: (), mut c: (), mut expected: ()) {
    if ((n) <= (unimplemented!())) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_10(mut s: (), mut n: (), mut c: (), mut expected: ()) {
    if ((n) <= (unimplemented!())) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_11(mut s: (), mut n: (), mut c: (), mut expected: ()) {
    if ((n) <= (unimplemented!())) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_string_12() {
    (unsafe { test_9(unimplemented!(), 0_(), ('a' as ()), unimplemented!()) });
    (unsafe {
        test_9(
            unimplemented!(),
            1_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"a".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            10_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
                { 10_() },
                { ('a' as ()) },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            100_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_2(
                { 100_() },
                { ('a' as ()) },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            0_(),
            ('a' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            2_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            5_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            15_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345aaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
            0_(),
            ('a' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
            10_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"1234567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
            50_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
            60_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890aaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_13)),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"not going to happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            21_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            22_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            23_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            24_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            29_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            30_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            31_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            32_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_9(
            unimplemented!(),
            33_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
}
pub unsafe fn test_string_14() {
    (unsafe { test_10(unimplemented!(), 0_(), ('a' as ()), unimplemented!()) });
    (unsafe {
        test_10(
            unimplemented!(),
            1_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"a".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            10_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_2(
                { 10_() },
                { ('a' as ()) },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            100_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_2(
                { 100_() },
                { ('a' as ()) },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            0_(),
            ('a' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            2_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            5_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            15_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345aaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
            0_(),
            ('a' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
            10_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"1234567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
            50_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
            60_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890aaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_15)),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"not going to happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            21_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            22_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            23_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            24_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            29_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            30_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            31_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            32_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_10(
            unimplemented!(),
            33_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
}
pub unsafe fn test_string_16() {
    (unsafe { test_11(unimplemented!(), 0_(), ('a' as ()), unimplemented!()) });
    (unsafe {
        test_11(
            unimplemented!(),
            1_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"a".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            unimplemented!(),
            10_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_2(
                { 10_() },
                { ('a' as ()) },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            unimplemented!(),
            100_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_2(
                { 100_() },
                { ('a' as ()) },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            0_(),
            ('a' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            2_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            5_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12345".as_ptr() },
                None,
            ),
            15_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12345aaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
            0_(),
            ('a' as ()),
            unimplemented!(),
        )
    });
    (unsafe {
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
            10_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"1234567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
            50_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890".as_ptr() },
                None,
            ),
            60_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"12345678901234567890123456789012345678901234567890aaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            unimplemented!(),
            (*std::cell::LazyCell::force_mut(&mut *&raw mut npos_17)),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"not going to happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            unimplemented!(),
            21_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            unimplemented!(),
            22_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            unimplemented!(),
            23_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            unimplemented!(),
            24_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            unimplemented!(),
            29_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            unimplemented!(),
            30_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            unimplemented!(),
            31_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            unimplemented!(),
            32_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_11(
            unimplemented!(),
            33_(),
            ('a' as ()),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".as_ptr() },
                None,
            ),
        )
    });
}
pub unsafe fn test_18() -> () {
    (unsafe { test_string_12() });
    (unsafe { test_string_14() });
    (unsafe { test_string_16() });
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
    (unsafe { test_18() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
