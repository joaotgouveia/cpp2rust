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
            if operator_lt_0(self as *const C2Int, other as *const C2Int) {
                std::cmp::Ordering::Less
            } else if operator_lt_0(other as *const C2Int, self as *const C2Int) {
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
            !(operator_lt_0(self as *const C2Int, other as *const C2Int))
                && !(operator_lt_0(other as *const C2Int, self as *const C2Int))
        }
    }
}
impl std::cmp::Eq for C2Int {}
impl Default for C2Int {
    fn default() -> Self {
        unsafe { C2Int::new_1() }
    }
}
pub unsafe fn operator_lt_1(mut rhs: (), lhs: *const C2Int) -> () {
    return ((rhs) < (unsafe { C2Int::get(&(*lhs)) }));
}
pub unsafe fn operator_lt_0(rhs: *const C2Int, lhs: *const C2Int) -> () {
    return ((unsafe { C2Int::get(&(*rhs)) }) < (unsafe { C2Int::get(&(*lhs)) }));
}
pub unsafe fn operator_lt_2(rhs: *const C2Int, mut lhs: ()) -> () {
    return ((unsafe { C2Int::get(&(*rhs)) }) < (lhs));
}
pub unsafe fn operator_eq_3(lhs: *const StoredType_int_, rhs: *const StoredType_int_) -> () {
    return (((*lhs).value_) == ((*rhs).value_));
}
pub unsafe fn operator_eq_4(lhs: *const StoredType_int_, rhs: *const SearchedType_int_) -> () {
    return (((*lhs).value_) == (unsafe { SearchedType_int_::get_value(&(*rhs)) }));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct StoredType_int_ {
    value_: (),
}
impl StoredType_int_ {
    pub unsafe fn new_1(mut value: ()) -> Self {
        let mut this = Self { value_: value };
        this
    }
    pub unsafe fn get_value(&self) -> () {
        return self.value_;
    }
}
impl std::cmp::PartialEq for StoredType_int_ {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            operator_eq_3(
                self as *const StoredType_int_,
                other as *const StoredType_int_,
            )
        }
    }
}
impl std::cmp::Eq for StoredType_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct SearchedType_int_ {
    value_: (),
    conversions_: *mut (),
}
impl SearchedType_int_ {
    pub unsafe fn new(mut value: (), mut counter: *mut ()) -> Self {
        let mut this = Self {
            value_: value,
            conversions_: counter,
        };
        this
    }
    pub unsafe fn to_StoredType_int_(&self) -> StoredType_int_ {
        (*self.conversions_).prefix_inc();
        return StoredType_int_::new_1({ self.value_ });
    }
    pub unsafe fn get_value(&self) -> () {
        return self.value_;
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct hash_impl {}
impl hash_impl {
    pub unsafe fn operator_call_1_int(&self, t: *const SearchedType_int_) -> () {
        return (unsafe { SearchedType_int_::get_value(&(*t)) });
    }
    pub unsafe fn operator_call_2_int(&self, t: *const StoredType_int_) -> () {
        return (unsafe { StoredType_int_::get_value(&(*t)) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct non_transparent_hash {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_hash {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_hash_final {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct transparent_equal_final {}
pub unsafe fn test_transparent_equal_range_5(mut c: ()) {
    let mut conversions: () = 0_();
    let mut iters: () = (unsafe {
        let mut ___k: SearchedType_int_ =
            SearchedType_int_::new({ 1_() }, { (&mut conversions as *mut ()) });
        c . equal_range_83_struct_SearchedType_int__struct_transparent_hash_struct_std_equal_to_void__targ  ( & mut ___k , )
    });
    unimplemented!();
    iters = (unsafe {
        let mut ___k: SearchedType_int_ =
            SearchedType_int_::new({ 2_() }, { (&mut conversions as *mut ()) });
        c . equal_range_83_struct_SearchedType_int__struct_transparent_hash_struct_std_equal_to_void__targ  ( & mut ___k , )
    });
    unimplemented!();
    iters = (unsafe {
        let mut ___k: SearchedType_int_ =
            SearchedType_int_::new({ 3_() }, { (&mut conversions as *mut ()) });
        c . equal_range_83_struct_SearchedType_int__struct_transparent_hash_struct_std_equal_to_void__targ  ( & mut ___k , )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_transparent_equal_range_6(c: ()) {
    let mut conversions: () = 0_();
    let mut iters: () = (unsafe {
        let mut ___k: SearchedType_int_ =
            SearchedType_int_::new({ 1_() }, { (&mut conversions as *mut ()) });
        c . equal_range_84_struct_SearchedType_int__struct_transparent_hash_struct_std_equal_to_void__targ  ( & mut ___k , )
    });
    unimplemented!();
    iters = (unsafe {
        let mut ___k: SearchedType_int_ =
            SearchedType_int_::new({ 2_() }, { (&mut conversions as *mut ()) });
        c . equal_range_84_struct_SearchedType_int__struct_transparent_hash_struct_std_equal_to_void__targ  ( & mut ___k , )
    });
    unimplemented!();
    iters = (unsafe {
        let mut ___k: SearchedType_int_ =
            SearchedType_int_::new({ 3_() }, { (&mut conversions as *mut ()) });
        c . equal_range_84_struct_SearchedType_int__struct_transparent_hash_struct_std_equal_to_void__targ  ( & mut ___k , )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_transparent_equal_range_7(mut c: ()) {
    let mut conversions: () = 0_();
    let mut iters: () = (unsafe {
        let mut ___k: SearchedType_int_ =
            SearchedType_int_::new({ 1_() }, { (&mut conversions as *mut ()) });
        c . equal_range_83_struct_SearchedType_int__struct_transparent_hash_final_struct_transparent_equal_final_targ  ( & mut ___k , )
    });
    unimplemented!();
    iters = (unsafe {
        let mut ___k: SearchedType_int_ =
            SearchedType_int_::new({ 2_() }, { (&mut conversions as *mut ()) });
        c . equal_range_83_struct_SearchedType_int__struct_transparent_hash_final_struct_transparent_equal_final_targ  ( & mut ___k , )
    });
    unimplemented!();
    iters = (unsafe {
        let mut ___k: SearchedType_int_ =
            SearchedType_int_::new({ 3_() }, { (&mut conversions as *mut ()) });
        c . equal_range_83_struct_SearchedType_int__struct_transparent_hash_final_struct_transparent_equal_final_targ  ( & mut ___k , )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_transparent_equal_range_8(c: ()) {
    let mut conversions: () = 0_();
    let mut iters: () = (unsafe {
        let mut ___k: SearchedType_int_ =
            SearchedType_int_::new({ 1_() }, { (&mut conversions as *mut ()) });
        c . equal_range_84_struct_SearchedType_int__struct_transparent_hash_final_struct_transparent_equal_final_targ  ( & mut ___k , )
    });
    unimplemented!();
    iters = (unsafe {
        let mut ___k: SearchedType_int_ =
            SearchedType_int_::new({ 2_() }, { (&mut conversions as *mut ()) });
        c . equal_range_84_struct_SearchedType_int__struct_transparent_hash_final_struct_transparent_equal_final_targ  ( & mut ___k , )
    });
    unimplemented!();
    iters = (unsafe {
        let mut ___k: SearchedType_int_ =
            SearchedType_int_::new({ 3_() }, { (&mut conversions as *mut ()) });
        c . equal_range_84_struct_SearchedType_int__struct_transparent_hash_final_struct_transparent_equal_final_targ  ( & mut ___k , )
    });
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_non_transparent_equal_range_9(mut c: ()) {
    let mut conversions: () = 0_();
    let mut iters: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    iters = unimplemented!();
    unimplemented!();
    unimplemented!();
    iters = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_non_transparent_equal_range_10(c: ()) {
    let mut conversions: () = 0_();
    let mut iters: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    iters = unimplemented!();
    unimplemented!();
    unimplemented!();
    iters = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_non_transparent_equal_range_11(mut c: ()) {
    let mut conversions: () = 0_();
    let mut iters: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    iters = unimplemented!();
    unimplemented!();
    unimplemented!();
    iters = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_non_transparent_equal_range_12(c: ()) {
    let mut conversions: () = 0_();
    let mut iters: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    iters = unimplemented!();
    unimplemented!();
    unimplemented!();
    iters = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_non_transparent_equal_range_13(mut c: ()) {
    let mut conversions: () = 0_();
    let mut iters: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    iters = unimplemented!();
    unimplemented!();
    unimplemented!();
    iters = unimplemented!();
    unimplemented!();
    unimplemented!();
}
pub unsafe fn test_non_transparent_equal_range_14(c: ()) {
    let mut conversions: () = 0_();
    let mut iters: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    iters = unimplemented!();
    unimplemented!();
    unimplemented!();
    iters = unimplemented!();
    unimplemented!();
    unimplemented!();
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
    {
        (unsafe { test_transparent_equal_range_5(unimplemented!()) });
        (unsafe { test_transparent_equal_range_6(unimplemented!()) });
    }
    {
        (unsafe { test_transparent_equal_range_7(unimplemented!()) });
        (unsafe { test_transparent_equal_range_8(unimplemented!()) });
    }
    {
        (unsafe { test_non_transparent_equal_range_9(unimplemented!()) });
        (unsafe { test_non_transparent_equal_range_10(unimplemented!()) });
    }
    {
        (unsafe { test_non_transparent_equal_range_11(unimplemented!()) });
        (unsafe { test_non_transparent_equal_range_12(unimplemented!()) });
    }
    {
        (unsafe { test_non_transparent_equal_range_13(unimplemented!()) });
        (unsafe { test_non_transparent_equal_range_14(unimplemented!()) });
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
