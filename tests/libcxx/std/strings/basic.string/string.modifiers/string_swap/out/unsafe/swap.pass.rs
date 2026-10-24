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
pub unsafe fn is_string_asan_correct_9(_a0: *const ()) -> () {
    return true;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct operator_hijacker {}
impl operator_hijacker {
    pub unsafe fn operator_lt(&self, _a0: *const operator_hijacker) -> () {
        return true;
    }
    pub unsafe fn operator_eq(&self, _a0: *const operator_hijacker) -> () {
        return true;
    }
    pub unsafe fn operator_call(&self) -> () {
        return 42_();
    }
}
impl std::cmp::Ord for operator_hijacker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_hijacker::operator_lt(self, other as *const operator_hijacker) {
                std::cmp::Ordering::Less
            } else if operator_hijacker::operator_lt(other, self as *const operator_hijacker) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for operator_hijacker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for operator_hijacker {
    fn eq(&self, other: &Self) -> bool {
        unsafe { operator_hijacker::operator_eq(self, other as *const operator_hijacker) }
    }
}
impl std::cmp::Eq for operator_hijacker {}
const _: () = assert!(
    true,
    "std::is_trivially_copyable<operator_hijacker>::value &&     //\n                  std::is_copy_constructible<operator_hijacker>::value && //\n                  std::is_move_constructible<operator_hijacker>::value && //\n                  std::is_copy_assignable<operator_hijacker>::value &&    //\n                  std::is_move_assignable<operator_hijacker>::value"
);
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct operator_hijacker_allocator_char_ {}
pub unsafe fn test_10(mut s1: (), mut s2: ()) {
    let mut s1_: () = unimplemented!();
    let mut s2_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_11(mut s1: (), mut s2: ()) {
    let mut s1_: () = unimplemented!();
    let mut s2_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_12(mut s1: (), mut s2: ()) {
    let mut s1_: () = unimplemented!();
    let mut s2_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_13(mut s1: (), mut s2: ()) {
    let mut s1_: () = unimplemented!();
    let mut s2_: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_string_14() {
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
    (unsafe { test_10(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_string_15() {
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
    (unsafe { test_11(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_string_16() {
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
    (unsafe { test_12(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_string_17() {
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
    (unsafe { test_13(unimplemented!(), unimplemented!()) });
}
pub unsafe fn test_18() -> () {
    (unsafe { test_string_14() });
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
