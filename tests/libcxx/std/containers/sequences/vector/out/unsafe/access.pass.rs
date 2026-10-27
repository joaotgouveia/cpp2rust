extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = ();
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_int_ref__int_ref_ {}
pub type anon_1 = ();
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_int_ref__const_int_ref_ {}
pub unsafe fn DoNotOptimize_2(value: *const ()) -> *const () {
    c"";
    return value;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct malloc_allocator_base {}
impl malloc_allocator_base {
    pub unsafe fn outstanding_alloc() -> () {
        unimplemented!();
        return ((*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4))
            .wrapping_sub((*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5))));
    }
    pub unsafe fn reset() {
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut disable_default_constructor_6)) = false;
        (*std::cell::LazyCell::force_mut(&mut *&raw mut outstanding_bytes_3)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut alloc_count_4)) = 0_();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dealloc_count_5)) = 0_();
    }
}
pub static mut outstanding_bytes_3: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut alloc_count_4: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut dealloc_count_5: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { 0_() });
pub static mut disable_default_constructor_6: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn operator_sub_7(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_eq_8(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) == (y.ptr_));
}
pub unsafe fn operator_ne_9(
    mut x: min_pointer_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return !(unsafe {
        let _x: min_pointer_int__std_integral_constant_unsigned_long__0__ = x;
        operator_eq_8(_x, y)
    });
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_int__std_integral_constant_unsigned_long__0__ {
    unsafe fn new_1(mut p: *mut ()) -> Self {
        let mut this = Self { ptr_: p };
        this
    }
    pub unsafe fn new_2(mut _a0: ()) -> Self {
        let mut this = Self {
            ptr_: std::ptr::null_mut(),
        };
        this
    }
    pub unsafe fn to_(&self) -> () {
        return !((self.ptr_).is_null());
    }
    pub unsafe fn operator_deref(&self) -> *mut () {
        return &mut (*self.ptr_);
    }
    pub unsafe fn operator_arrow(&self) -> *mut () {
        return self.ptr_;
    }
    pub unsafe fn operator_inc_8(
        &mut self,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_inc();
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_dec_10(
        &mut self,
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_.prefix_dec();
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_add(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_sub_assign(
        &mut self,
        mut n: (),
    ) -> *mut min_pointer_int__std_integral_constant_unsigned_long__0__ {
        self.ptr_ = (self.ptr_).wrapping_sub(n as usize);
        return &mut (*(self as *mut min_pointer_int__std_integral_constant_unsigned_long__0__));
    }
    pub unsafe fn operator_add(
        &self,
        mut n: (),
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
        mut n: (),
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
    pub unsafe fn operator_index(&self, mut n: ()) -> *mut () {
        return &mut (*self.ptr_.offset((n) as isize));
    }
}
impl std::cmp::PartialEq for min_pointer_int__std_integral_constant_unsigned_long__0__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_eq_8(self.clone(), other.clone()) }
    }
}
impl std::cmp::Eq for min_pointer_int__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
pub unsafe fn operator_sub_10(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return (((x.ptr_ as usize - y.ptr_ as usize) / ::std::mem::size_of::<()>()) as () as ());
}
pub unsafe fn operator_ne_11(
    mut x: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
    mut y: min_pointer_const_int__std_integral_constant_unsigned_long__0__,
) -> () {
    return ((x.ptr_) != (y.ptr_));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    ptr_: *const (),
}
impl min_pointer_const_int__std_integral_constant_unsigned_long__0__ {
    pub unsafe fn new_1(mut p: min_pointer_int__std_integral_constant_unsigned_long__0__) -> Self {
        let mut this = Self {
            ptr_: (p.ptr_).cast_const(),
        };
        this
    }
    pub unsafe fn operator_deref(&self) -> *const () {
        return &(*self.ptr_);
    }
    pub unsafe fn operator_index(&self, mut n: ()) -> *const () {
        return &(*self.ptr_.offset((n) as isize));
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
pub struct min_allocator_int_ {}
impl min_allocator_int_ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> min_pointer_int__std_integral_constant_unsigned_long__0__ {
        return min_pointer_int__std_integral_constant_unsigned_long__0__::new_1({
            unimplemented!()
        });
    }
    pub unsafe fn deallocate(
        &mut self,
        mut p: min_pointer_int__std_integral_constant_unsigned_long__0__,
        mut n: (),
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
    pub unsafe fn allocate(&mut self, mut n: ()) -> *mut () {
        let mut memory: *mut () = unimplemented!();
        {
            (unsafe {
                let ___s: () = (memory as ());
                memset_12(
                    ___s,
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
                    let ___s: () = (p as ());
                    memset_12(
                        ___s,
                        0_(),
                        (::std::mem::size_of::<()>() as ()).wrapping_mul(n),
                    )
                });
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
pub unsafe fn make_13(mut size: (), mut start: ()) -> () {
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (size)) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub unsafe fn make_14(mut size: (), mut start: ()) -> () {
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (size)) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub unsafe fn make_15(mut size: (), mut start: ()) -> () {
    let mut c: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (size)) {
        unimplemented!();
        i.prefix_inc();
    }
    return unimplemented!();
}
pub unsafe fn test_get_basic_16(c: *mut (), mut start_value: ()) {
    let n: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_get_basic_17(c: *const (), mut start_value: ()) {
    let n: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_get_basic_18(c: *mut (), mut start_value: ()) {
    let n: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_get_basic_19(c: *const (), mut start_value: ()) {
    let n: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_get_basic_20(c: *mut (), mut start_value: ()) {
    let n: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_get_basic_21(c: *const (), mut start_value: ()) {
    let n: () = unimplemented!();
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_get_22() {
    let mut start_value: () = 35_();
    let mut c: () = (unsafe { make_13(10_(), start_value) });
    let cc: *const () = &c;
    (unsafe { test_get_basic_16(&mut c, start_value) });
    (unsafe {
        let _c: *const () = cc;
        let _start_value: () = start_value;
        test_get_basic_17(_c, _start_value)
    });
}
pub unsafe fn test_get_23() {
    let mut start_value: () = 35_();
    let mut c: () = (unsafe { make_14(10_(), start_value) });
    let cc: *const () = &c;
    (unsafe { test_get_basic_18(&mut c, start_value) });
    (unsafe {
        let _c: *const () = cc;
        let _start_value: () = start_value;
        test_get_basic_19(_c, _start_value)
    });
}
pub unsafe fn test_get_24() {
    let mut start_value: () = 35_();
    let mut c: () = (unsafe { make_15(10_(), start_value) });
    let cc: *const () = &c;
    (unsafe { test_get_basic_20(&mut c, start_value) });
    (unsafe {
        let _c: *const () = cc;
        let _start_value: () = start_value;
        test_get_basic_21(_c, _start_value)
    });
}
pub unsafe fn test_set_25() {
    let mut start_value: () = 35_();
    let n: () = 10_();
    let mut c: () = (unsafe { make_13(n, start_value) });
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        (*unimplemented!()) = (((start_value) + (i)) + (1_()));
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        (*unimplemented!()) = (((start_value) + (i)) + (2_()));
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    (*unimplemented!()) = ((start_value) + (3_()));
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = (((start_value) + (n)) + (2_()));
    unimplemented!();
}
pub unsafe fn test_set_26() {
    let mut start_value: () = 35_();
    let n: () = 10_();
    let mut c: () = (unsafe { make_14(n, start_value) });
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        (*unimplemented!()) = (((start_value) + (i)) + (1_()));
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        (*unimplemented!()) = (((start_value) + (i)) + (2_()));
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    (*unimplemented!()) = ((start_value) + (3_()));
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = (((start_value) + (n)) + (2_()));
    unimplemented!();
}
pub unsafe fn test_set_27() {
    let mut start_value: () = 35_();
    let n: () = 10_();
    let mut c: () = (unsafe { make_15(n, start_value) });
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        (*unimplemented!()) = (((start_value) + (i)) + (1_()));
        unimplemented!();
        i.prefix_inc();
    }
    let mut i: () = 0_();
    'loop_: while ((i) < (n)) {
        unimplemented!();
        (*unimplemented!()) = (((start_value) + (i)) + (2_()));
        unimplemented!();
        i.prefix_inc();
    }
    unimplemented!();
    (*unimplemented!()) = ((start_value) + (3_()));
    unimplemented!();
    unimplemented!();
    (*unimplemented!()) = (((start_value) + (n)) + (2_()));
    unimplemented!();
}
pub unsafe fn test_28() {
    (unsafe { test_get_22() });
    (unsafe { test_set_25() });
    let mut c: () = unimplemented!();
    let cc: *const () = &c;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_29() {
    (unsafe { test_get_23() });
    (unsafe { test_set_26() });
    let mut c: () = unimplemented!();
    let cc: *const () = &c;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn test_30() {
    (unsafe { test_get_24() });
    (unsafe { test_set_27() });
    let mut c: () = unimplemented!();
    let cc: *const () = &c;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
}
pub unsafe fn tests_31() -> () {
    (unsafe { test_28() });
    (unsafe { test_29() });
    (unsafe { test_30() });
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
    (unsafe { tests_31() });
    const _: () = assert!(true, "tests()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_3);
    std::cell::LazyCell::force(&*&raw const alloc_count_4);
    std::cell::LazyCell::force(&*&raw const dealloc_count_5);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_6);
}
