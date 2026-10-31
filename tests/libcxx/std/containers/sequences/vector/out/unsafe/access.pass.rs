extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ref__int_ref_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ref__const_int_ref_ {}
pub unsafe fn DoNotOptimize_2(value: *const *mut ::libc::c_void) -> *const *mut ::libc::c_void {
    c"";
    return value;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> usize {
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4))
                >= (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)))
        );
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5))));
    }
    pub unsafe fn reset() {
        assert!(((unsafe { malloc_allocator_base::outstanding_alloc() }) == (0_usize)));
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_6)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_3)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4)) = 0_usize;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) = 0_usize;
    }
}
pub static mut outstanding_bytes_3: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut alloc_count_4: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut dealloc_count_5: std::cell::LazyCell<usize> =
    std::cell::LazyCell::new(|| unsafe { 0_usize });
pub static mut disable_default_constructor_6: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_7(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> isize {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<i32>()) as i64 as isize);
}
pub unsafe fn operator_eq_8(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_9(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_8(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut i32,
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut i32) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn to_bool(&self) -> bool {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut i32 {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_int__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_int__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_int__std_integral_constant_unsigned_long__0__::operator_add_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
    pub unsafe fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_int__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_int__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_int__std_integral_constant_unsigned_long__0__::operator_sub_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
impl UnsafeCppAdvance for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn cpp_inc(&mut self) {
        self.operator_inc_8();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut ::libc::c_void,
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_eq_10(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> bool {
    return ((x.ptr_) == (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(
    min_pointer_int__std_integral_constant_unsigned_long__0__,
    new_1,
    value
)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const i32,
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const i32 {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: isize,
    ) -> *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self
            as *mut min_pointer_const_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub(
        &self,
        mut n: isize,
    ) -> min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
        let mut tmp: min_pointer_const_int__std_integral_constant_unsigned_long__0__ =
            (*(self as *const min_pointer_const_int__std_integral_constant_unsigned_long__0__));
        (unsafe {
            min_pointer_const_int__std_integral_constant_unsigned_long__0__::operator_sub_assign(
                &mut tmp, n,
            )
        });
        return tmp;
    }
}
impl std::cmp::PartialEq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_10(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_const_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_void__std_integral_constant_unsigned_long__0__ {
    ptr_: *const ::libc::c_void,
}
impl min_pointer_const_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: usize,
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: usize,
    ) {
        unimplemented!();
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct safe_allocator_int_ {}
impl safe_allocator_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: usize) -> *mut i32 {
        let mut memory: *mut i32 = unimplemented!();
        {
            {
                let byte_0 = (memory as *mut ::libc::c_void) as *mut u8;
                for offset in
                    0..((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                {
                    *byte_0.offset(offset as isize) = 0 as u8;
                }
                (memory as *mut ::libc::c_void)
            };
        }
        return memory;
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut i32, mut n: usize) {
        {
            (unsafe {
                let mut _value: *mut ::libc::c_void = ({
                    let byte_0 = (p as *mut ::libc::c_void) as *mut u8;
                    for offset in
                        0..((::std::mem::size_of::<i32>() as u64).wrapping_mul((n as u64)) as usize)
                    {
                        *byte_0.offset(offset as isize) = 0 as u8;
                    }
                    (p as *mut ::libc::c_void)
                } as *mut ::libc::c_void);
                DoNotOptimize_2(&mut _value)
            });
        }
        unimplemented!();
    }
}
impl Default for safe_allocator_int_ {
    fn default() -> Self {
        unsafe { safe_allocator_int_::new() }
    }
}
pub unsafe fn make_11(mut size: i32, mut start: i32) -> Vec<i32> {
    let mut c: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (size)) {
        {
            let __a1 = ((start) + (i));
            c.push(__a1)
        };
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
}
pub unsafe fn make_12(mut size: i32, mut start: i32) -> Vec<i32> {
    let mut c: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (size)) {
        {
            let __a1 = ((start) + (i));
            c.push(__a1)
        };
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
}
pub unsafe fn make_13(mut size: i32, mut start: i32) -> Vec<i32> {
    let mut c: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    'loop_: while ((i) < (size)) {
        {
            let __a1 = ((start) + (i));
            c.push(__a1)
        };
        i.prefix_inc();
    }
    return std::mem::take(&mut c);
}
pub unsafe fn test_get_basic_14(c: *mut Vec<i32>, mut start_value: i32) {
    let n: i32 = ((*c).len() as i32);
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(((*(&mut (*c)).as_mut_ptr().add((i as usize) as usize)) == ((start_value) + (i))));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(
            ((*if (i as usize) as usize >= (&mut (*c)).len() {
                panic!("out of bounds access")
            } else {
                (&mut (*c)).as_mut_ptr().add((i as usize) as usize)
            }) == ((start_value) + (i)))
        );
        i.prefix_inc();
    }
    assert!(((*((&mut (*c)).first_mut().unwrap())) == (start_value)));
    assert!(((*((&mut (*c)).last_mut().unwrap())) == (((start_value) + (n)) - (1))));
}
pub unsafe fn test_get_basic_15(c: *const Vec<i32>, mut start_value: i32) {
    let n: i32 = ((*c).len() as i32);
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(((*(&(*c)).as_ptr().add((i as usize) as usize)) == ((start_value) + (i))));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(
            ((*{
                let __idx = (i as usize);
                if __idx as usize >= (&(*c)).len() {
                    panic!("out of bounds access")
                } else {
                    (&(*c)).as_ptr().add(__idx as usize)
                }
            }) == ((start_value) + (i)))
        );
        i.prefix_inc();
    }
    assert!(((*unimplemented!()) == (start_value)));
    assert!(((*((&*(*c)).last().unwrap())) == (((start_value) + (n)) - (1))));
}
pub unsafe fn test_get_basic_16(c: *mut Vec<i32>, mut start_value: i32) {
    let n: i32 = ((*c).len() as i32);
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(((*(&mut (*c)).as_mut_ptr().add((i as usize) as usize)) == ((start_value) + (i))));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(
            ((*if (i as usize) as usize >= (&mut (*c)).len() {
                panic!("out of bounds access")
            } else {
                (&mut (*c)).as_mut_ptr().add((i as usize) as usize)
            }) == ((start_value) + (i)))
        );
        i.prefix_inc();
    }
    assert!(((*((&mut (*c)).first_mut().unwrap())) == (start_value)));
    assert!(((*((&mut (*c)).last_mut().unwrap())) == (((start_value) + (n)) - (1))));
}
pub unsafe fn test_get_basic_17(c: *const Vec<i32>, mut start_value: i32) {
    let n: i32 = ((*c).len() as i32);
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(((*(&(*c)).as_ptr().add((i as usize) as usize)) == ((start_value) + (i))));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(
            ((*{
                let __idx = (i as usize);
                if __idx as usize >= (&(*c)).len() {
                    panic!("out of bounds access")
                } else {
                    (&(*c)).as_ptr().add(__idx as usize)
                }
            }) == ((start_value) + (i)))
        );
        i.prefix_inc();
    }
    assert!(((*unimplemented!()) == (start_value)));
    assert!(((*((&*(*c)).last().unwrap())) == (((start_value) + (n)) - (1))));
}
pub unsafe fn test_get_basic_18(c: *mut Vec<i32>, mut start_value: i32) {
    let n: i32 = ((*c).len() as i32);
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(((*(&mut (*c)).as_mut_ptr().add((i as usize) as usize)) == ((start_value) + (i))));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(
            ((*if (i as usize) as usize >= (&mut (*c)).len() {
                panic!("out of bounds access")
            } else {
                (&mut (*c)).as_mut_ptr().add((i as usize) as usize)
            }) == ((start_value) + (i)))
        );
        i.prefix_inc();
    }
    assert!(((*((&mut (*c)).first_mut().unwrap())) == (start_value)));
    assert!(((*((&mut (*c)).last_mut().unwrap())) == (((start_value) + (n)) - (1))));
}
pub unsafe fn test_get_basic_19(c: *const Vec<i32>, mut start_value: i32) {
    let n: i32 = ((*c).len() as i32);
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(((*(&(*c)).as_ptr().add((i as usize) as usize)) == ((start_value) + (i))));
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(
            ((*{
                let __idx = (i as usize);
                if __idx as usize >= (&(*c)).len() {
                    panic!("out of bounds access")
                } else {
                    (&(*c)).as_ptr().add(__idx as usize)
                }
            }) == ((start_value) + (i)))
        );
        i.prefix_inc();
    }
    assert!(((*unimplemented!()) == (start_value)));
    assert!(((*((&*(*c)).last().unwrap())) == (((start_value) + (n)) - (1))));
}
pub unsafe fn test_get_20() {
    let mut start_value: i32 = 35;
    let mut c: Vec<i32> = (unsafe { make_11(10, start_value) });
    let cc: *const Vec<i32> = &c;
    (unsafe { test_get_basic_14(&mut c, start_value) });
    (unsafe {
        let _c: *const Vec<i32> = cc;
        let _start_value: i32 = start_value;
        test_get_basic_15(_c, _start_value)
    });
}
pub unsafe fn test_get_21() {
    let mut start_value: i32 = 35;
    let mut c: Vec<i32> = (unsafe { make_12(10, start_value) });
    let cc: *const Vec<i32> = &c;
    (unsafe { test_get_basic_16(&mut c, start_value) });
    (unsafe {
        let _c: *const Vec<i32> = cc;
        let _start_value: i32 = start_value;
        test_get_basic_17(_c, _start_value)
    });
}
pub unsafe fn test_get_22() {
    let mut start_value: i32 = 35;
    let mut c: Vec<i32> = (unsafe { make_13(10, start_value) });
    let cc: *const Vec<i32> = &c;
    (unsafe { test_get_basic_18(&mut c, start_value) });
    (unsafe {
        let _c: *const Vec<i32> = cc;
        let _start_value: i32 = start_value;
        test_get_basic_19(_c, _start_value)
    });
}
pub unsafe fn test_set_23() {
    let mut start_value: i32 = 35;
    let n: i32 = 10;
    let mut c: Vec<i32> = (unsafe { make_11(n, start_value) });
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(((*(&mut c).as_mut_ptr().add((i as usize) as usize)) == ((start_value) + (i))));
        (*(&mut c).as_mut_ptr().add((i as usize) as usize)) = (((start_value) + (i)) + (1));
        assert!(
            ((*(&mut c).as_mut_ptr().add((i as usize) as usize)) == (((start_value) + (i)) + (1)))
        );
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(
            ((*if (i as usize) as usize >= (&mut c).len() {
                panic!("out of bounds access")
            } else {
                (&mut c).as_mut_ptr().add((i as usize) as usize)
            }) == (((start_value) + (i)) + (1)))
        );
        (*if (i as usize) as usize >= (&mut c).len() {
            panic!("out of bounds access")
        } else {
            (&mut c).as_mut_ptr().add((i as usize) as usize)
        }) = (((start_value) + (i)) + (2));
        assert!(
            ((*if (i as usize) as usize >= (&mut c).len() {
                panic!("out of bounds access")
            } else {
                (&mut c).as_mut_ptr().add((i as usize) as usize)
            }) == (((start_value) + (i)) + (2)))
        );
        i.prefix_inc();
    }
    assert!(((*((&mut c).first_mut().unwrap())) == ((start_value) + (2))));
    (*((&mut c).first_mut().unwrap())) = ((start_value) + (3));
    assert!(((*((&mut c).first_mut().unwrap())) == ((start_value) + (3))));
    assert!(((*((&mut c).last_mut().unwrap())) == (((start_value) + (n)) + (1))));
    (*((&mut c).last_mut().unwrap())) = (((start_value) + (n)) + (2));
    assert!(((*((&mut c).last_mut().unwrap())) == (((start_value) + (n)) + (2))));
}
pub unsafe fn test_set_24() {
    let mut start_value: i32 = 35;
    let n: i32 = 10;
    let mut c: Vec<i32> = (unsafe { make_12(n, start_value) });
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(((*(&mut c).as_mut_ptr().add((i as usize) as usize)) == ((start_value) + (i))));
        (*(&mut c).as_mut_ptr().add((i as usize) as usize)) = (((start_value) + (i)) + (1));
        assert!(
            ((*(&mut c).as_mut_ptr().add((i as usize) as usize)) == (((start_value) + (i)) + (1)))
        );
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(
            ((*if (i as usize) as usize >= (&mut c).len() {
                panic!("out of bounds access")
            } else {
                (&mut c).as_mut_ptr().add((i as usize) as usize)
            }) == (((start_value) + (i)) + (1)))
        );
        (*if (i as usize) as usize >= (&mut c).len() {
            panic!("out of bounds access")
        } else {
            (&mut c).as_mut_ptr().add((i as usize) as usize)
        }) = (((start_value) + (i)) + (2));
        assert!(
            ((*if (i as usize) as usize >= (&mut c).len() {
                panic!("out of bounds access")
            } else {
                (&mut c).as_mut_ptr().add((i as usize) as usize)
            }) == (((start_value) + (i)) + (2)))
        );
        i.prefix_inc();
    }
    assert!(((*((&mut c).first_mut().unwrap())) == ((start_value) + (2))));
    (*((&mut c).first_mut().unwrap())) = ((start_value) + (3));
    assert!(((*((&mut c).first_mut().unwrap())) == ((start_value) + (3))));
    assert!(((*((&mut c).last_mut().unwrap())) == (((start_value) + (n)) + (1))));
    (*((&mut c).last_mut().unwrap())) = (((start_value) + (n)) + (2));
    assert!(((*((&mut c).last_mut().unwrap())) == (((start_value) + (n)) + (2))));
}
pub unsafe fn test_set_25() {
    let mut start_value: i32 = 35;
    let n: i32 = 10;
    let mut c: Vec<i32> = (unsafe { make_13(n, start_value) });
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(((*(&mut c).as_mut_ptr().add((i as usize) as usize)) == ((start_value) + (i))));
        (*(&mut c).as_mut_ptr().add((i as usize) as usize)) = (((start_value) + (i)) + (1));
        assert!(
            ((*(&mut c).as_mut_ptr().add((i as usize) as usize)) == (((start_value) + (i)) + (1)))
        );
        i.prefix_inc();
    }
    let mut i: i32 = 0;
    'loop_: while ((i) < (n)) {
        assert!(
            ((*if (i as usize) as usize >= (&mut c).len() {
                panic!("out of bounds access")
            } else {
                (&mut c).as_mut_ptr().add((i as usize) as usize)
            }) == (((start_value) + (i)) + (1)))
        );
        (*if (i as usize) as usize >= (&mut c).len() {
            panic!("out of bounds access")
        } else {
            (&mut c).as_mut_ptr().add((i as usize) as usize)
        }) = (((start_value) + (i)) + (2));
        assert!(
            ((*if (i as usize) as usize >= (&mut c).len() {
                panic!("out of bounds access")
            } else {
                (&mut c).as_mut_ptr().add((i as usize) as usize)
            }) == (((start_value) + (i)) + (2)))
        );
        i.prefix_inc();
    }
    assert!(((*((&mut c).first_mut().unwrap())) == ((start_value) + (2))));
    (*((&mut c).first_mut().unwrap())) = ((start_value) + (3));
    assert!(((*((&mut c).first_mut().unwrap())) == ((start_value) + (3))));
    assert!(((*((&mut c).last_mut().unwrap())) == (((start_value) + (n)) + (1))));
    (*((&mut c).last_mut().unwrap())) = (((start_value) + (n)) + (2));
    assert!(((*((&mut c).last_mut().unwrap())) == (((start_value) + (n)) + (2))));
}
pub unsafe fn test_26() {
    (unsafe { test_get_20() });
    (unsafe { test_set_23() });
    let mut c: Vec<i32> = Vec::new();
    let cc: *const Vec<i32> = &c;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_27() {
    (unsafe { test_get_21() });
    (unsafe { test_set_24() });
    let mut c: Vec<i32> = Vec::new();
    let cc: *const Vec<i32> = &c;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_28() {
    (unsafe { test_get_22() });
    (unsafe { test_set_25() });
    let mut c: Vec<i32> = Vec::new();
    let cc: *const Vec<i32> = &c;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn tests_29() -> bool {
    (unsafe { test_26() });
    (unsafe { test_27() });
    (unsafe { test_28() });
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { tests_29() });
    const _: () = assert!(true, "tests()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_3);
    std::cell::LazyCell::force(&*&raw const alloc_count_4);
    std::cell::LazyCell::force(&*&raw const dealloc_count_5);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_6);
}
