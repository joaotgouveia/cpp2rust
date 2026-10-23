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
pub struct min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__ {
    ptr_: *mut (),
}
impl min_pointer_std_pair_const_Static__int___std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_void__std_integral_constant_unsigned_long__0__ {
    ptr_: (),
}
impl min_pointer_void__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_Static__std_integral_constant_unsigned_long__0__ {
    ptr_: *mut Static,
}
impl min_pointer_Static__std_integral_constant_unsigned_long__0__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { ptr_ : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { ptr_ : *mut min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__ , }
impl min_pointer_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0____std_integral_constant_unsigned_long__0__ { }
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_Static_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_std_pair_const_Static__int__ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct min_allocator_min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0___
{}
pub static mut value_4: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (((((((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_5))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_5)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_6)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_6)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_7)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_7)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_8)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_8))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct node_compatibility_table_int__int__std_less_int___std_less_int___std_hash_int___std_hash_int___std_equal_to_int___std_equal_to_int___std_allocator_int___std_allocator_std_pair_const_int__int___
{}
pub static mut value_9: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (((((((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_5))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_5)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_6)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_6)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_7)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_7)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_8)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_8))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct node_compatibility_table_int__int__std_less_int___my_compare_int___std_hash_int___my_hash_int___std_equal_to_int___my_equal_int___std_allocator_int___std_allocator_std_pair_const_int__int___
{}
pub static mut value_10: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe {
    (((((((*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_11))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_11)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_12)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_12)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_13)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_13)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_14)))
        && (*std::cell::LazyCell::force_mut(&mut *&raw mut is_same_v_14))
});
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct node_compatibility_table_Static__int__my_compare_Static___std_less_Static___my_hash_Static___std_hash_Static___my_equal_Static___std_equal_to_Static___min_allocator_Static___min_allocator_std_pair_const_Static__int___
{}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct my_hash_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct my_hash_Static_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct my_compare_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct my_compare_Static_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct my_equal_int_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct my_equal_Static_ {}
#[repr(C)]
#[derive(FnPtrArg, Default)]
pub struct Static {}
const _: () = assert!(
    true,
    "node_compatibility_table< int,\n                              int,\n                              std::less<int>,\n                              std::less<int>,\n                              std::hash<int>,\n                              std::hash<int>,\n                              std::equal_to<int>,\n                              std::equal_to<int>,\n                              std::allocator<int>,\n                              std::allocator<std::pair<const int, int>>>::value"
);
const _: () = assert!(
    true,
    "node_compatibility_table<int,\n                             int,\n                             std::less<int>,\n                             my_compare<int>,\n                             std::hash<int>,\n                             my_hash<int>,\n                             std::equal_to<int>,\n                             my_equal<int>,\n                             std::allocator<int>,\n                             std::allocator<std::pair<const int, int>>>::value"
);
const _: () = assert!(
    true,
    "node_compatibility_table< Static,\n                              int,\n                              my_compare<Static>,\n                              std::less<Static>,\n                              my_hash<Static>,\n                              std::hash<Static>,\n                              my_equal<Static>,\n                              std::equal_to<Static>,\n                              min_allocator<Static>,\n                              min_allocator<std::pair<const Static, int>>>::value"
);
pub unsafe fn test_node_handle_operations_15() -> () {
    let mut c: () = unimplemented!();
    let mut nt1: () = unimplemented!();
    let mut nt2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_node_handle_operations_16() -> () {
    let mut c: () = unimplemented!();
    let mut nt1: () = unimplemented!();
    let mut nt2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_node_handle_operations_17() -> () {
    let mut c: () = unimplemented!();
    let mut nt1: () = unimplemented!();
    let mut nt2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_node_handle_operations_18() -> () {
    let mut c: () = unimplemented!();
    let mut nt1: () = unimplemented!();
    let mut nt2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_node_handle_operations_multi_19() -> () {
    let mut c: () = unimplemented!();
    let mut nt1: () = unimplemented!();
    let mut nt2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_node_handle_operations_multi_20() -> () {
    let mut c: () = unimplemented!();
    let mut nt1: () = unimplemented!();
    let mut nt2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_node_handle_operations_multi_21() -> () {
    let mut c: () = unimplemented!();
    let mut nt1: () = unimplemented!();
    let mut nt2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_node_handle_operations_multi_22() -> () {
    let mut c: () = unimplemented!();
    let mut nt1: () = unimplemented!();
    let mut nt2: () = unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return true;
}
pub unsafe fn test_typedef_23() -> () {
    return true;
}
pub unsafe fn test_typedef_24() -> () {
    return true;
}
pub unsafe fn test_typedef_25() -> () {
    return true;
}
pub unsafe fn test_typedef_26() -> () {
    return true;
}
pub unsafe fn test_insert_return_type_27() -> () {
    (unsafe { test_typedef_23() });
    return true;
}
pub unsafe fn test_insert_return_type_28() -> () {
    (unsafe { test_typedef_24() });
    return true;
}
pub unsafe fn test_insert_return_type_29() -> () {
    (unsafe { test_typedef_25() });
    return true;
}
pub unsafe fn test_insert_return_type_30() -> () {
    (unsafe { test_typedef_26() });
    return true;
}
pub unsafe fn test_31() -> () {
    (unsafe { test_node_handle_operations_15() });
    (unsafe { test_node_handle_operations_multi_19() });
    (unsafe { test_node_handle_operations_16() });
    {
        (unsafe { test_node_handle_operations_multi_20() });
        (unsafe { test_node_handle_operations_17() });
        (unsafe { test_node_handle_operations_multi_21() });
        (unsafe { test_node_handle_operations_18() });
        (unsafe { test_node_handle_operations_multi_22() });
    }
    (unsafe { test_insert_return_type_27() });
    (unsafe { test_insert_return_type_28() });
    {
        (unsafe { test_insert_return_type_29() });
        (unsafe { test_insert_return_type_30() });
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
    (unsafe { test_31() });
    return 0_();
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_std___hash_value_type_Static__int___min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct min_pointer_std___hash_node_base_min_pointer_std___hash_node_Static__min_pointer_void__std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0_____std_integral_constant_unsigned_long__0__;
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const outstanding_bytes_0);
    std::cell::LazyCell::force(&*&raw const alloc_count_1);
    std::cell::LazyCell::force(&*&raw const dealloc_count_2);
    std::cell::LazyCell::force(&*&raw const disable_default_constructor_3);
    std::cell::LazyCell::force(&*&raw const value_4);
    std::cell::LazyCell::force(&*&raw const value_9);
    std::cell::LazyCell::force(&*&raw const value_10);
}
