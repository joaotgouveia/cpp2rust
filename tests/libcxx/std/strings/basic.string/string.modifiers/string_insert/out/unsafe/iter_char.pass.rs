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
pub unsafe fn operator_sub_6(
    mut x: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_7(
    mut x: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_char__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
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
impl std::cmp::PartialEq for min_pointer_const_char__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_7(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_char__std_integral_constant_unsigned_long__0__ {}
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
                memset_8(
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
                    memset_8(
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
pub unsafe fn is_string_asan_correct_9(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_10(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn is_string_asan_correct_11(_a0: *const ()) -> () {
    return true;
}
pub unsafe fn test_12(s: *mut (), mut p: (), mut c: (), mut expected: ()) {
    unimplemented!();
    let mut sufficient_cap: () = ((unimplemented!()) < (unimplemented!()));
    let mut pos: () = (unimplemented!() as ());
    let mut i: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    if sufficient_cap {
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_13(s: *mut (), mut p: (), mut c: (), mut expected: ()) {
    unimplemented!();
    let mut sufficient_cap: () = ((unimplemented!()) < (unimplemented!()));
    let mut pos: () = unimplemented!();
    let mut i: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    if sufficient_cap {
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_14(s: *mut (), mut p: (), mut c: (), mut expected: ()) {
    unimplemented!();
    let mut sufficient_cap: () = ((unimplemented!()) < (unimplemented!()));
    let mut pos: () = (unimplemented!() as ());
    let mut i: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    if sufficient_cap {
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_string_15() {
    let mut s: () = unimplemented!();
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_0: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_0 })
        };
        test_12(_s, _p, ('1' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_1: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_1 })
        };
        test_12(_s, _p, ('a' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_2: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_2 })
        };
        test_12(_s, _p, ('b' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_3: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_3 })
        };
        test_12(_s, _p, ('c' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_4: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_4 })
        };
        test_12(_s, _p, ('d' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_5: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_5 })
        };
        test_12(_s, _p, ('2' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_6: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_6 })
        };
        test_12(_s, _p, ('3' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_7: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_7 })
        };
        test_12(_s, _p, ('4' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_8: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_8 })
        };
        test_12(_s, _p, ('5' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_9: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_9 })
        };
        test_12(_s, _p, ('6' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_10: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_10 })
        };
        test_12(_s, _p, ('7' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_11: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_11 })
        };
        test_12(_s, _p, ('A' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_12: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_12 })
        };
        test_12(_s, _p, ('B' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_13: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_13 })
        };
        test_12(_s, _p, ('C' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_14: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_14 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_15: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_15 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_16: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_16 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_17: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_17 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_18: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_18 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_19: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_19 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_20: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_20 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_21: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_21 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_22: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_22 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_23: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_23 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_24: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_24 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_25: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_25 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_26: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_26 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_27: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_27 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_28: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_28 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_29: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_29 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_30: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_30 })
        };
        test_12(_s, _p, ('x' as ()), unimplemented!())
    });
}
pub unsafe fn test_string_16() {
    let mut s: () = unimplemented!();
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_31: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_31 },
            )
        };
        test_13(_s, _p, ('1' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_32: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_32 },
            )
        };
        test_13(_s, _p, ('a' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_33: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_33 },
            )
        };
        test_13(_s, _p, ('b' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_34: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_34 },
            )
        };
        test_13(_s, _p, ('c' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_35: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_35 },
            )
        };
        test_13(_s, _p, ('d' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_36: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_36 },
            )
        };
        test_13(_s, _p, ('2' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_37: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_37 },
            )
        };
        test_13(_s, _p, ('3' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_38: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_38 },
            )
        };
        test_13(_s, _p, ('4' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_39: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_39 },
            )
        };
        test_13(_s, _p, ('5' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_40: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_40 },
            )
        };
        test_13(_s, _p, ('6' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_41: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_41 },
            )
        };
        test_13(_s, _p, ('7' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_42: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_42 },
            )
        };
        test_13(_s, _p, ('A' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_43: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_43 },
            )
        };
        test_13(_s, _p, ('B' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_44: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_44 },
            )
        };
        test_13(_s, _p, ('C' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_45: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_45 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_46: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_46 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_47: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_47 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_48: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_48 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_49: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_49 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_50: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_50 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_51: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_51 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_52: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_52 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_53: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_53 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_54: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_54 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_55: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_55 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_56: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_56 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_57: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_57 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_58: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_58 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_59: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_59 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_60: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_60 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_61: () = unimplemented!();
            std___wrap_iter_min_pointer_const_char__std_integral_constant_unsigned_long__0___::new_1(
                { &mut __tmp_61 },
            )
        };
        test_13(_s, _p, ('x' as ()), unimplemented!())
    });
}
pub unsafe fn test_string_17() {
    let mut s: () = unimplemented!();
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_62: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_62 })
        };
        test_14(_s, _p, ('1' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_63: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_63 })
        };
        test_14(_s, _p, ('a' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_64: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_64 })
        };
        test_14(_s, _p, ('b' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_65: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_65 })
        };
        test_14(_s, _p, ('c' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_66: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_66 })
        };
        test_14(_s, _p, ('d' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_67: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_67 })
        };
        test_14(_s, _p, ('2' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_68: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_68 })
        };
        test_14(_s, _p, ('3' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_69: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_69 })
        };
        test_14(_s, _p, ('4' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_70: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_70 })
        };
        test_14(_s, _p, ('5' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_71: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_71 })
        };
        test_14(_s, _p, ('6' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_72: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_72 })
        };
        test_14(_s, _p, ('7' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_73: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_73 })
        };
        test_14(_s, _p, ('A' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_74: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_74 })
        };
        test_14(_s, _p, ('B' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_75: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_75 })
        };
        test_14(_s, _p, ('C' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_76: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_76 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_77: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_77 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_78: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_78 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_79: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_79 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_80: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_80 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_81: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_81 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_82: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_82 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_83: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_83 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_84: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_84 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_85: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_85 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_86: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_86 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_87: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_87 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_88: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_88 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_89: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_89 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_90: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_90 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_91: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_91 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
    (unsafe {
        let _s: *mut () = &mut s;
        let _p: () = {
            let mut __tmp_92: () = unimplemented!();
            std___wrap_iter_const_char_ptr_::new_1({ &mut __tmp_92 })
        };
        test_14(_s, _p, ('x' as ()), unimplemented!())
    });
}
pub unsafe fn test_18() -> () {
    (unsafe { test_string_15() });
    (unsafe { test_string_16() });
    (unsafe { test_string_17() });
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
