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
pub unsafe fn test_6(
    mut s: (),
    mut pos: (),
    mut n1: (),
    mut str: *const (),
    mut n2: (),
    mut expected: (),
) {
    let old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if ((pos) <= (old_size)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut xlen: () = (*unimplemented!());
        let mut rlen: () = n2;
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_7(
    mut s: (),
    mut pos: (),
    mut n1: (),
    mut str: *const (),
    mut n2: (),
    mut expected: (),
) {
    let old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if ((pos) <= (old_size)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut xlen: () = (*unimplemented!());
        let mut rlen: () = n2;
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test_8(
    mut s: (),
    mut pos: (),
    mut n1: (),
    mut str: *const (),
    mut n2: (),
    mut expected: (),
) {
    let old_size: () = unimplemented!();
    let mut s0: () = unimplemented!();
    if ((pos) <= (old_size)) {
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut xlen: () = (*unimplemented!());
        let mut rlen: () = n2;
        unimplemented!();
        unimplemented!();
    }
}
pub unsafe fn test0_9() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test0_10() -> () {
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test0_11() -> () {
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test1_12() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test1_13() -> () {
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test1_14() -> () {
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test2_15() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test2_16() -> () {
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test2_17() -> () {
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test3_18() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test3_19() -> () {
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test3_20() -> () {
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            3_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            2_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            4_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test4_21() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test4_22() -> () {
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test4_23() -> () {
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            6_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test5_24() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test5_25() -> () {
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test5_26() -> () {
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            8_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test6_27() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test6_28() -> () {
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test6_29() -> () {
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            4_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test7_30() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test7_31() -> () {
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test7_32() -> () {
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            5_(),
            6_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            9_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            11_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test8_33() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test8_34() -> () {
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test8_35() -> () {
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            0_(),
            21_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test9_36() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test9_37() -> () {
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test9_38() -> () {
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            18_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            19_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            1_(),
            20_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test10_39() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test10_40() -> () {
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test10_41() -> () {
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            5_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            9_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            10_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            10_(),
            11_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test11_42() -> () {
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_6(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test11_43() -> () {
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_7(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test11_44() -> () {
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            19_(),
            2_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            20_(),
            1_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            2_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            4_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"12345".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            5_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            9_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"1234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            0_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            1_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            10_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            19_(),
            unimplemented!(),
        )
    });
    (unsafe {
        test_8(
            unimplemented!(),
            21_(),
            0_(),
            c"12345678901234567890".as_ptr(),
            20_(),
            unimplemented!(),
        )
    });
    return true;
}
pub unsafe fn test_45() {
    (unsafe { test0_9() });
    (unsafe { test1_12() });
    (unsafe { test2_15() });
    (unsafe { test3_18() });
    (unsafe { test4_21() });
    (unsafe { test5_24() });
    (unsafe { test6_27() });
    (unsafe { test7_30() });
    (unsafe { test8_33() });
    (unsafe { test9_36() });
    (unsafe { test10_39() });
    (unsafe { test11_42() });
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
pub unsafe fn test_46() {
    (unsafe { test0_10() });
    (unsafe { test1_13() });
    (unsafe { test2_16() });
    (unsafe { test3_19() });
    (unsafe { test4_22() });
    (unsafe { test5_25() });
    (unsafe { test6_28() });
    (unsafe { test7_31() });
    (unsafe { test8_34() });
    (unsafe { test9_37() });
    (unsafe { test10_40() });
    (unsafe { test11_43() });
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
pub unsafe fn test_47() {
    (unsafe { test0_11() });
    (unsafe { test1_14() });
    (unsafe { test2_17() });
    (unsafe { test3_20() });
    (unsafe { test4_23() });
    (unsafe { test5_26() });
    (unsafe { test6_29() });
    (unsafe { test7_32() });
    (unsafe { test8_35() });
    (unsafe { test9_38() });
    (unsafe { test10_41() });
    (unsafe { test11_44() });
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
    (unsafe { test_45() });
    (unsafe { test_46() });
    (unsafe { test_47() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_1);
    std::cell::LazyCell::force(&*&raw const alloc_count_2);
    std::cell::LazyCell::force(&*&raw const dealloc_count_3);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_4);
}
