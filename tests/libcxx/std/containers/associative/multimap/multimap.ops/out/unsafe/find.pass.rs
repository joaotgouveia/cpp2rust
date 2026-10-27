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
pub struct min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_int__double___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_int__double__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct PrivateConstructor {
    val: (),
}
impl PrivateConstructor {
    pub unsafe fn make(mut v: ()) -> PrivateConstructor {
        return PrivateConstructor::new({ v });
    }
    pub unsafe fn get(&self) -> () {
        return self.val;
    }
    unsafe fn new(mut v: ()) -> Self {
        let mut this = Self { val: v };
        this
    }
}
impl std::cmp::Ord for PrivateConstructor {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_4(
                self as *const PrivateConstructor,
                other as *const PrivateConstructor,
            ) {
                std::cmp::Ordering::Less
            } else if operator_lt_4(
                other as *const PrivateConstructor,
                self as *const PrivateConstructor,
            ) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for PrivateConstructor {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for PrivateConstructor {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_4(
                self as *const PrivateConstructor,
                other as *const PrivateConstructor,
            )) && !(operator_lt_4(
                other as *const PrivateConstructor,
                self as *const PrivateConstructor,
            ))
        }
    }
}
impl std::cmp::Eq for PrivateConstructor {}
pub unsafe fn operator_lt_4(lhs: *const PrivateConstructor, rhs: *const PrivateConstructor) -> () {
    return ((unsafe { PrivateConstructor::get(&(*lhs)) })
        < (unsafe { PrivateConstructor::get(&(*rhs)) }));
}
pub unsafe fn operator_lt_5(lhs: *const PrivateConstructor, mut rhs: ()) -> () {
    return ((unsafe { PrivateConstructor::get(&(*lhs)) }) < (rhs));
}
pub unsafe fn operator_lt_6(mut lhs: (), rhs: *const PrivateConstructor) -> () {
    return ((lhs) < (unsafe { PrivateConstructor::get(&(*rhs)) }));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_not_referenceable {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_nonempty_is_transparent {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_nonempty {
    pub pad_: transparent_less_nonempty_is_transparent,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_no_type {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_private {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_less_not_a_type {
    pub is_transparent: (),
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct C2Int {
    i_: (),
}
impl C2Int {
    pub unsafe fn new_1() -> Self {
        let mut this = Self { i_: 0_() };
        this
    }
    pub unsafe fn new_2(mut i: ()) -> Self {
        let mut this = Self { i_: i };
        this
    }
    pub unsafe fn get(&self) -> () {
        return self.i_;
    }
}
impl std::cmp::Ord for C2Int {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe {
            if operator_lt_7(self as *const C2Int, other as *const C2Int) {
                std::cmp::Ordering::Less
            } else if operator_lt_7(other as *const C2Int, self as *const C2Int) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        }
    }
}
impl std::cmp::PartialOrd for C2Int {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::PartialEq for C2Int {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            !(operator_lt_7(self as *const C2Int, other as *const C2Int))
                && !(operator_lt_7(other as *const C2Int, self as *const C2Int))
        }
    }
}
impl std::cmp::Eq for C2Int {}
impl Default for C2Int {
    fn default() -> Self {
        unsafe { C2Int::new_1() }
    }
}
pub unsafe fn operator_lt_8(mut rhs: (), lhs: *const C2Int) -> () {
    return ((rhs) < (unsafe { C2Int::get(&(*lhs)) }));
}
pub unsafe fn operator_lt_7(rhs: *const C2Int, lhs: *const C2Int) -> () {
    return ((unsafe { C2Int::get(&(*rhs)) }) < (unsafe { C2Int::get(&(*lhs)) }));
}
pub unsafe fn operator_lt_9(rhs: *const C2Int, mut lhs: ()) -> () {
    return ((unsafe { C2Int::get(&(*rhs)) }) < (lhs));
}
pub unsafe fn iter_in_range_10(mut first: (), mut last: (), mut to_find: ()) -> () {
    'loop_: while unimplemented!() {
        if unimplemented!() {
            return true;
        }
        unimplemented!();
    }
    return false;
}
pub unsafe fn iter_in_range_11(mut first: (), mut last: (), mut to_find: ()) -> () {
    'loop_: while unimplemented!() {
        if unimplemented!() {
            return true;
        }
        unimplemented!();
    }
    return false;
}
pub unsafe fn iter_in_range_12(mut first: (), mut last: (), mut to_find: ()) -> () {
    'loop_: while unimplemented!() {
        if unimplemented!() {
            return true;
        }
        unimplemented!();
    }
    return false;
}
pub unsafe fn iter_in_range_13(mut first: (), mut last: (), mut to_find: ()) -> () {
    'loop_: while unimplemented!() {
        if unimplemented!() {
            return true;
        }
        unimplemented!();
    }
    return false;
}
pub unsafe fn iter_in_range_14(mut first: (), mut last: (), mut to_find: ()) -> () {
    'loop_: while unimplemented!() {
        if unimplemented!() {
            return true;
        }
        unimplemented!();
    }
    return false;
}
pub unsafe fn iter_in_range_15(mut first: (), mut last: (), mut to_find: ()) -> () {
    'loop_: while unimplemented!() {
        if unimplemented!() {
            return true;
        }
        unimplemented!();
    }
    return false;
}
pub unsafe fn test_16() -> () {
    {
        {
            let mut ar: [(); 9] = [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ];
            let mut m: () = unimplemented!();
            let mut r: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut ar: [(); 9] = [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ];
            let m: () = unimplemented!();
            let mut r: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut arr: [(); 9] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        {
            let mut ar: [(); 9] = [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ];
            let mut m: () = unimplemented!();
            let mut r: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
        {
            let mut ar: [(); 9] = [
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
                unimplemented!(),
            ];
            let m: () = unimplemented!();
            let mut r: () = unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
            unimplemented!();
        }
    }
    {
        let mut ar: [(); 9] = [
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
            unimplemented!(),
        ];
        let mut m: () = unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
    }
    {
        let mut m: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        let mut r: () = unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
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
    (unsafe { test_16() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
}
