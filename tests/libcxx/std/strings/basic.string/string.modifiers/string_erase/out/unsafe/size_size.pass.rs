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
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
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
    pub unsafe fn operator_add(
        &self,
        mut n: (),
    ) -> min_pointer_char__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_char__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_char__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_char__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
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
pub unsafe fn test_9(mut s: (), mut pos: (), mut n: (), mut expected: ()) {
    let old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if ((pos) <= (old_size)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_10(mut s: (), mut pos: (), mut n: (), mut expected: ()) {
    let old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if ((pos) <= (old_size)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_11(mut s: (), mut pos: (), mut n: (), mut expected: ()) {
    let old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if ((pos) <= (old_size)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_12(mut s: (), mut pos: (), mut expected: ()) {
    let old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if ((pos) <= (old_size)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_13(mut s: (), mut pos: (), mut expected: ()) {
    let old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if ((pos) <= (old_size)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_14(mut s: (), mut pos: (), mut expected: ()) {
    let old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if ((pos) <= (old_size)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_15(mut s: (), mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_16(mut s: (), mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_17(mut s: (), mut expected: ()) {
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_18() {
    (unsafe {
        test_9(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
            0_(),
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
            1_(),
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
            1_(),
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
            0_(),
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
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"bcde".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"cde".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"e".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
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
            6_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"acde".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"ade".as_ptr() },
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
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"ae".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abde".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abe".as_ptr() },
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
            3_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"ab".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"ab".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcd".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcd".as_ptr() },
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
            1_(),
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
            6_(),
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
            0_(),
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
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"bcdefghij".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"fghij".as_ptr() },
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
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"j".as_ptr() },
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
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
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
            11_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"acdefghij".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"afghij".as_ptr() },
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
            8_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"aj".as_ptr() },
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
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdeghij".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdehij".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdej".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
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
            6_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghi".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghi".as_ptr() },
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
            1_(),
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
            11_(),
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
            0_(),
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
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"bcdefghijklmnopqrst".as_ptr() },
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
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"klmnopqrst".as_ptr() },
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
            19_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"t".as_ptr() },
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
            20_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
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
            21_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"acdefghijklmnopqrst".as_ptr() },
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
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"aklmnopqrst".as_ptr() },
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
            18_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"at".as_ptr() },
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
            19_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            20_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijlmnopqrst".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijpqrst".as_ptr() },
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
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijt".as_ptr() },
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
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
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
            11_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrs".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrs".as_ptr() },
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
            1_(),
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
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"a".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            2_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"ab".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            4_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcd".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            6_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"a".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            5_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            9_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghi".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            11_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            0_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            1_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"a".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            10_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            19_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrs".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            20_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_12(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            21_(),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_15(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_15(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_15(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_15(
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            std_basic_string_char__std_char_traits_char___std_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
}
pub unsafe fn test_string_19() {
    (unsafe {
        test_10(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
            0_(),
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
            1_(),
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
            1_(),
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
            0_(),
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
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"bcde".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"cde".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"e".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
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
            6_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"acde".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"ade".as_ptr() },
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
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"ae".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abde".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abe".as_ptr() },
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
            3_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"ab".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"ab".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcd".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcd".as_ptr() },
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
            1_(),
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
            6_(),
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
            0_(),
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
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"bcdefghij".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"fghij".as_ptr() },
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
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"j".as_ptr() },
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
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
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
            11_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"acdefghij".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"afghij".as_ptr() },
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
            8_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"aj".as_ptr() },
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
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdeghij".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdehij".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdej".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
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
            6_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghi".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghi".as_ptr() },
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
            1_(),
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
            11_(),
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
            0_(),
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
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"bcdefghijklmnopqrst".as_ptr() },
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
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"klmnopqrst".as_ptr() },
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
            19_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"t".as_ptr() },
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
            20_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
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
            21_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"acdefghijklmnopqrst".as_ptr() },
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
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"aklmnopqrst".as_ptr() },
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
            18_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"at".as_ptr() },
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
            19_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            20_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijlmnopqrst".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijpqrst".as_ptr() },
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
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijt".as_ptr() },
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
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
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
            11_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrs".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrs".as_ptr() },
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
            1_(),
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
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"a".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            2_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"ab".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            4_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcd".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            6_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"a".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            5_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            9_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghi".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            11_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            0_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            1_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"a".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            10_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            19_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrs".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            20_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_13(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            21_(),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_16(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_16(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_16(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_16(
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            std_basic_string_char__std_char_traits_char___min_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
}
pub unsafe fn test_string_20() {
    (unsafe {
        test_11(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
            0_(),
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
            1_(),
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
            1_(),
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
            0_(),
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
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"bcde".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"cde".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"e".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
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
            6_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"acde".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"ade".as_ptr() },
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
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"ae".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abde".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abe".as_ptr() },
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
            3_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"ab".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"ab".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcd".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcd".as_ptr() },
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
            1_(),
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
            6_(),
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
            0_(),
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
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"bcdefghij".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"fghij".as_ptr() },
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
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"j".as_ptr() },
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
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
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
            11_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"acdefghij".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"afghij".as_ptr() },
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
            8_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"aj".as_ptr() },
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
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdeghij".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdehij".as_ptr() },
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
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdej".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcde".as_ptr() },
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
            6_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcde".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghi".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghi".as_ptr() },
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
            1_(),
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
            11_(),
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
            0_(),
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
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"bcdefghijklmnopqrst".as_ptr() },
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
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"klmnopqrst".as_ptr() },
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
            19_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"t".as_ptr() },
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
            20_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
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
            21_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"acdefghijklmnopqrst".as_ptr() },
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
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"aklmnopqrst".as_ptr() },
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
            18_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"at".as_ptr() },
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
            19_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            20_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"a".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijlmnopqrst".as_ptr() },
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
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijpqrst".as_ptr() },
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
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijt".as_ptr() },
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
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
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
            11_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
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
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijklmnopqrs".as_ptr() },
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
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijklmnopqrs".as_ptr() },
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
            1_(),
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
            21_(),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"a".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            2_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"ab".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            4_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcd".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            6_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"a".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            5_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            9_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghi".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            11_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            0_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            1_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"a".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            10_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            19_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijklmnopqrs".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            20_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_14(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            21_(),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"can\'t happen".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_17(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_17(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcde".as_ptr() },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_17(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghij".as_ptr() },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
    (unsafe {
        test_17(
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"abcdefghijklmnopqrst".as_ptr() },
                None,
            ),
            std_basic_string_char__std_char_traits_char___safe_allocator_char__::new_1(
                { c"".as_ptr() },
                None,
            ),
        )
    });
}
pub unsafe fn test_21() -> () {
    (unsafe { test_string_18() });
    (unsafe { test_string_19() });
    (unsafe { test_string_20() });
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
    (unsafe { test_21() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
