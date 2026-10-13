extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_0 {}
impl Clone for allocator_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_0 {}
impl allocator_0 {}
impl allocator_0 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_1 {}
impl Clone for allocator_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_1 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_1 {
    pub unsafe fn copy_from(__a: *const allocator_1) -> Self {
        unimplemented!()
    }
}
impl allocator_1 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_1) -> *mut allocator_1 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_2 {}
impl Clone for allocator_2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_2 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_2 {
    pub unsafe fn copy_from(__a: *const allocator_2) -> Self {
        unimplemented!()
    }
}
impl allocator_2 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_2) -> *mut allocator_2 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_3 {}
impl Clone for allocator_3 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_3 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_3 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_3 {
    pub unsafe fn copy_from(__a: *const allocator_3) -> Self {
        unimplemented!()
    }
}
impl allocator_3 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_3) -> *mut allocator_3 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_4 {}
impl Clone for allocator_4 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_4 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_4 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_4 {
    pub unsafe fn copy_from(__a: *const allocator_4) -> Self {
        unimplemented!()
    }
}
impl allocator_4 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_4) -> *mut allocator_4 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_5 {}
impl Clone for allocator_5 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_5 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_5 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_5 {
    pub unsafe fn copy_from(__a: *const allocator_5) -> Self {
        unimplemented!()
    }
}
impl allocator_5 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_5) -> *mut allocator_5 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_6 {}
impl Clone for allocator_6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_6 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_6 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_6 {
    pub unsafe fn copy_from(__a: *const allocator_6) -> Self {
        unimplemented!()
    }
}
impl allocator_6 {
    pub unsafe fn copy_assign(&mut self, _a0: *const allocator_6) -> *mut allocator_6 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_7 {}
impl Clone for allocator_traits_7 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_7 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_7 {
    pub unsafe fn allocate_1(__a: *mut allocator_3, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl allocator_traits_7 {
    pub unsafe fn deallocate(__a: *mut allocator_3, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_7 {
    pub unsafe fn max_size(__a: *const allocator_3) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_7 {
    pub unsafe fn select_on_container_copy_construction(__rhs: *const allocator_3) -> allocator_3 {
        unimplemented!()
    }
}
impl allocator_traits_7 {
    pub unsafe fn construct_8(__a: *mut allocator_3, mut __p: *mut i32, __args: *mut i32) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_9 {}
impl Clone for allocator_traits_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_9 {
    pub unsafe fn allocate_1(__a: *mut allocator_4, mut __n: u64) -> *mut vector_10 {
        unimplemented!()
    }
}
impl allocator_traits_9 {
    pub unsafe fn deallocate(__a: *mut allocator_4, mut __p: *mut vector_10, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_9 {
    pub unsafe fn max_size(__a: *const allocator_4) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_9 {
    pub unsafe fn construct_11(
        __a: *mut allocator_4,
        mut __p: *mut vector_10,
        __args: *const vector_10,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_9 {
    pub unsafe fn construct_12(
        __a: *mut allocator_4,
        mut __p: *mut vector_10,
        __args: *mut vector_10,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_9 {
    pub unsafe fn destroy_13(__a: *mut allocator_4, mut __p: *mut vector_10) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_14 {}
impl Clone for allocator_traits_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_14 {
    pub unsafe fn allocate_1(__a: *mut allocator_5, mut __n: u64) -> *mut Explicit {
        unimplemented!()
    }
}
impl allocator_traits_14 {
    pub unsafe fn deallocate(__a: *mut allocator_5, mut __p: *mut Explicit, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_14 {
    pub unsafe fn max_size(__a: *const allocator_5) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_14 {
    pub unsafe fn construct_15(
        __a: *mut allocator_5,
        mut __p: *mut Explicit,
        __args: *const Explicit,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_14 {
    pub unsafe fn construct_16(
        __a: *mut allocator_5,
        mut __p: *mut Explicit,
        __args: *mut Explicit,
    ) {
        unimplemented!()
    }
}
impl allocator_traits_14 {
    pub unsafe fn destroy_17(__a: *mut allocator_5, mut __p: *mut Explicit) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct allocator_traits_18 {}
impl Clone for allocator_traits_18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_18 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_18 {
    pub unsafe fn allocate_1(__a: *mut allocator_6, mut __n: u64) -> *mut Buffer {
        unimplemented!()
    }
}
impl allocator_traits_18 {
    pub unsafe fn deallocate(__a: *mut allocator_6, mut __p: *mut Buffer, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_18 {
    pub unsafe fn max_size(__a: *const allocator_6) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_18 {
    pub unsafe fn construct_19(__a: *mut allocator_6, mut __p: *mut Buffer, __args: *mut Buffer) {
        unimplemented!()
    }
}
impl allocator_traits_18 {
    pub unsafe fn destroy_20(__a: *mut allocator_6, mut __p: *mut Buffer) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_exception {}
impl Clone for std_exception {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_exception {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn copy_from(_a0: *const std_exception) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_exception) -> *mut std_exception {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn move_from(_a0: *mut std_exception) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub unsafe fn move_assign(&mut self, _a0: *mut std_exception) -> *mut std_exception {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_bad_alloc {}
impl Clone for std_bad_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_bad_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn copy_from(_a0: *const std_bad_alloc) -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_bad_alloc) -> *mut std_bad_alloc {
        unimplemented!()
    }
}
pub type std_align_val_t = usize;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_nothrow_t {}
impl Clone for std_nothrow_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_nothrow_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_nothrow_t {}
impl std_nothrow_t {}
pub static mut nothrow_21: std::cell::LazyCell<std_nothrow_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_22 {}
impl Clone for integral_constant_22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_22 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_integral_constant_bool__true__value_23: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_22 {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_22 {}
impl integral_constant_22 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_24 {}
impl Clone for integral_constant_24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_24 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std_integral_constant_bool__false__value_25: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
impl integral_constant_24 {}
impl integral_constant_24 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_26 {}
impl Clone for integral_constant_26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_26 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct integral_constant_27 {}
impl Clone for integral_constant_27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_27 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_28 {}
impl Clone for enable_if_28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_28 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_29 {}
impl Clone for enable_if_29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_30 {}
impl Clone for enable_if_30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_30 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_31 {}
impl Clone for enable_if_31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_32 {}
impl Clone for enable_if_32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_33 {}
impl Clone for enable_if_33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_34 {}
impl Clone for enable_if_34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct enable_if_35 {}
impl Clone for enable_if_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __conditional_36 {}
impl Clone for __conditional_36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __conditional_37 {}
impl Clone for __conditional_37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_38 {}
impl Clone for __type_identity_38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_39 {}
impl Clone for __type_identity_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_40 {}
impl Clone for __type_identity_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_41 {}
impl Clone for __type_identity_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_42 {}
impl Clone for __type_identity_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_43 {}
impl Clone for __type_identity_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_44 {}
impl Clone for __type_identity_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_45 {}
impl Clone for __type_identity_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_46 {}
impl Clone for __type_identity_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_47 {}
impl Clone for __type_identity_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_48 {}
impl Clone for __type_identity_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_49 {}
impl Clone for __type_identity_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_50 {}
impl Clone for __type_identity_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_51 {}
impl Clone for __type_identity_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_52 {}
impl Clone for __type_identity_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __type_identity_53 {}
impl Clone for __type_identity_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__54 {}
impl Clone for __or__54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__55 {}
impl Clone for __or__55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__56 {}
impl Clone for __or__56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__57 {}
impl Clone for __or__57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__58 {}
impl Clone for __or__58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__59 {}
impl Clone for __or__59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__60 {}
impl Clone for __or__60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__61 {}
impl Clone for __or__61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__62 {}
impl Clone for __or__62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__63 {}
impl Clone for __or__63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__64 {}
impl Clone for __or__64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__65 {}
impl Clone for __or__65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__66 {}
impl Clone for __or__66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__67 {}
impl Clone for __or__67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__68 {}
impl Clone for __or__68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__69 {}
impl Clone for __or__69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__70 {}
impl Clone for __or__70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__71 {}
impl Clone for __or__71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__72 {}
impl Clone for __or__72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__73 {}
impl Clone for __or__73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__74 {}
impl Clone for __or__74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__75 {}
impl Clone for __or__75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__76 {}
impl Clone for __or__76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__77 {}
impl Clone for __or__77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__78 {}
impl Clone for __or__78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__79 {}
impl Clone for __or__79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__80 {}
impl Clone for __or__80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__81 {}
impl Clone for __or__81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__82 {}
impl Clone for __or__82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__83 {}
impl Clone for __or__83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__84 {}
impl Clone for __or__84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __or__85 {}
impl Clone for __or__85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__86 {}
impl Clone for __and__86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__87 {}
impl Clone for __and__87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__88 {}
impl Clone for __and__88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__89 {}
impl Clone for __and__89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__90 {}
impl Clone for __and__90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__91 {}
impl Clone for __and__91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__92 {}
impl Clone for __and__92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__93 {}
impl Clone for __and__93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__94 {}
impl Clone for __and__94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__95 {}
impl Clone for __and__95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__96 {}
impl Clone for __and__96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__97 {}
impl Clone for __and__97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__98 {}
impl Clone for __and__98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__99 {}
impl Clone for __and__99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__100 {}
impl Clone for __and__100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__101 {}
impl Clone for __and__101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__102 {}
impl Clone for __and__102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__103 {}
impl Clone for __and__103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__104 {}
impl Clone for __and__104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__105 {}
impl Clone for __and__105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__106 {}
impl Clone for __and__106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __and__107 {}
impl Clone for __and__107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__108 {}
impl Clone for __not__108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__109 {}
impl Clone for __not__109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__110 {}
impl Clone for __not__110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__111 {}
impl Clone for __not__111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__112 {}
impl Clone for __not__112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __not__113 {}
impl Clone for __not__113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_114 {}
impl Clone for is_reference_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_115 {}
impl Clone for is_reference_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_116 {}
impl Clone for is_reference_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_117 {}
impl Clone for is_reference_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_118 {}
impl Clone for is_reference_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_119 {}
impl Clone for is_reference_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_120 {}
impl Clone for is_reference_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_121 {}
impl Clone for is_reference_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_122 {}
impl Clone for is_reference_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_123 {}
impl Clone for is_reference_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_124 {}
impl Clone for is_reference_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_125 {}
impl Clone for is_reference_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_126 {}
impl Clone for is_reference_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_127 {}
impl Clone for is_reference_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_128 {}
impl Clone for is_reference_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_reference_129 {}
impl Clone for is_reference_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_130 {}
impl Clone for is_function_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_131 {}
impl Clone for is_function_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_132 {}
impl Clone for is_function_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_133 {}
impl Clone for is_function_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_134 {}
impl Clone for is_function_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_135 {}
impl Clone for is_function_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_136 {}
impl Clone for is_function_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_137 {}
impl Clone for is_function_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_138 {}
impl Clone for is_function_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_139 {}
impl Clone for is_function_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_140 {}
impl Clone for is_function_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_141 {}
impl Clone for is_function_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_142 {}
impl Clone for is_function_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_function_143 {}
impl Clone for is_function_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_144 {}
impl Clone for is_void_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_145 {}
impl Clone for is_void_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_146 {}
impl Clone for is_void_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_147 {}
impl Clone for is_void_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_148 {}
impl Clone for is_void_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_149 {}
impl Clone for is_void_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_150 {}
impl Clone for is_void_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_151 {}
impl Clone for is_void_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_152 {}
impl Clone for is_void_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_153 {}
impl Clone for is_void_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_154 {}
impl Clone for is_void_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_155 {}
impl Clone for is_void_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_156 {}
impl Clone for is_void_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_157 {}
impl Clone for is_void_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_void_158 {}
impl Clone for is_void_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_159 {}
impl Clone for remove_cv_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_160 {}
impl Clone for remove_cv_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_161 {}
impl Clone for remove_cv_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_162 {}
impl Clone for remove_cv_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_163 {}
impl Clone for remove_cv_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_164 {}
impl Clone for remove_cv_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_165 {}
impl Clone for remove_cv_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_166 {}
impl Clone for remove_cv_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_167 {}
impl Clone for remove_cv_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_cv_168 {}
impl Clone for remove_cv_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_169 {}
impl Clone for is_const_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_170 {}
impl Clone for is_const_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_171 {}
impl Clone for is_const_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_172 {}
impl Clone for is_const_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_const_173 {}
impl Clone for is_const_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_174 {}
impl Clone for __is_array_unknown_bounds_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_175 {}
impl Clone for __is_array_unknown_bounds_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_176 {}
impl Clone for __is_array_unknown_bounds_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_177 {}
impl Clone for __is_array_unknown_bounds_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_178 {}
impl Clone for __is_array_unknown_bounds_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_179 {}
impl Clone for __is_array_unknown_bounds_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_180 {}
impl Clone for __is_array_unknown_bounds_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_181 {}
impl Clone for __is_array_unknown_bounds_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_182 {}
impl Clone for __is_array_unknown_bounds_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_183 {}
impl Clone for __is_array_unknown_bounds_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_184 {}
impl Clone for __is_array_unknown_bounds_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_185 {}
impl Clone for __is_array_unknown_bounds_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_186 {}
impl Clone for __is_array_unknown_bounds_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_array_unknown_bounds_187 {}
impl Clone for __is_array_unknown_bounds_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_188 {}
impl Clone for __is_integral_helper_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_189 {}
impl Clone for __is_integral_helper_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_190 {}
impl Clone for __is_integral_helper_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_191 {}
impl Clone for __is_integral_helper_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_192 {}
impl Clone for __is_integral_helper_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_193 {}
impl Clone for __is_integral_helper_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_194 {}
impl Clone for __is_integral_helper_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_195 {}
impl Clone for __is_integral_helper_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_196 {}
impl Clone for __is_integral_helper_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integral_helper_197 {}
impl Clone for __is_integral_helper_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_198 {}
impl Clone for is_integral_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_199 {}
impl Clone for is_integral_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_200 {}
impl Clone for is_integral_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_201 {}
impl Clone for is_integral_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_202 {}
impl Clone for is_integral_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_203 {}
impl Clone for is_integral_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_204 {}
impl Clone for is_integral_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_205 {}
impl Clone for is_integral_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_206 {}
impl Clone for is_integral_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_integral_207 {}
impl Clone for is_integral_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_floating_point_helper_208 {}
impl Clone for __is_floating_point_helper_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_floating_point_helper_209 {}
impl Clone for __is_floating_point_helper_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_floating_point_helper_210 {}
impl Clone for __is_floating_point_helper_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_floating_point_211 {}
impl Clone for is_floating_point_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_floating_point_212 {}
impl Clone for is_floating_point_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_floating_point_213 {}
impl Clone for is_floating_point_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_214 {}
impl Clone for is_pointer_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_215 {}
impl Clone for is_pointer_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_216 {}
impl Clone for is_pointer_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_217 {}
impl Clone for is_pointer_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_218 {}
impl Clone for is_pointer_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_219 {}
impl Clone for is_pointer_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_220 {}
impl Clone for is_pointer_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_pointer_221 {}
impl Clone for is_pointer_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_rvalue_reference_222 {}
impl Clone for is_rvalue_reference_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_rvalue_reference_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_member_object_pointer_223 {}
impl Clone for is_member_object_pointer_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_object_pointer_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_member_function_pointer_224 {}
impl Clone for is_member_function_pointer_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_function_pointer_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_enum_225 {}
impl Clone for is_enum_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_enum_226 {}
impl Clone for is_enum_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_enum_227 {}
impl Clone for is_enum_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_null_pointer_228 {}
impl Clone for is_null_pointer_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_null_pointer_229 {}
impl Clone for is_null_pointer_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_null_pointer_230 {}
impl Clone for is_null_pointer_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_231 {}
impl Clone for is_arithmetic_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_232 {}
impl Clone for is_arithmetic_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_233 {}
impl Clone for is_arithmetic_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_arithmetic_234 {}
impl Clone for is_arithmetic_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_member_pointer_235 {}
impl Clone for is_member_pointer_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_member_pointer_236 {}
impl Clone for is_member_pointer_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_member_pointer_237 {}
impl Clone for is_member_pointer_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_238 {}
impl Clone for is_scalar_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_239 {}
impl Clone for is_scalar_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_240 {}
impl Clone for is_scalar_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_scalar_241 {}
impl Clone for is_scalar_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_242 {}
impl Clone for is_same_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_243 {}
impl Clone for is_same_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_244 {}
impl Clone for is_same_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_245 {}
impl Clone for is_same_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_246 {}
impl Clone for is_same_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_247 {}
impl Clone for is_same_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_248 {}
impl Clone for is_same_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_same_249 {}
impl Clone for is_same_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_250 {}
impl Clone for is_volatile_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_251 {}
impl Clone for is_volatile_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_252 {}
impl Clone for is_volatile_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_253 {}
impl Clone for is_volatile_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_volatile_254 {}
impl Clone for is_volatile_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_empty_255 {}
impl Clone for is_empty_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_empty_256 {}
impl Clone for is_empty_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_all_extents_257 {}
impl Clone for remove_all_extents_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_all_extents_258 {}
impl Clone for remove_all_extents_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_all_extents_259 {}
impl Clone for remove_all_extents_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___do_is_destructible_impl {}
impl Clone for std___do_is_destructible_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_is_destructible_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_impl_260 {}
impl Clone for __is_destructible_impl_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_impl_261 {}
impl Clone for __is_destructible_impl_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_impl_262 {}
impl Clone for __is_destructible_impl_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_safe_263 {}
impl Clone for __is_destructible_safe_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_safe_264 {}
impl Clone for __is_destructible_safe_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_safe_265 {}
impl Clone for __is_destructible_safe_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_destructible_safe_266 {}
impl Clone for __is_destructible_safe_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___do_is_nt_destructible_impl {}
impl Clone for std___do_is_nt_destructible_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_is_nt_destructible_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nt_destructible_impl_267 {}
impl Clone for __is_nt_destructible_impl_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_impl_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nt_destructible_impl_268 {}
impl Clone for __is_nt_destructible_impl_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_impl_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nt_destructible_impl_269 {}
impl Clone for __is_nt_destructible_impl_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_impl_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nt_destructible_safe_270 {}
impl Clone for __is_nt_destructible_safe_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nt_destructible_safe_271 {}
impl Clone for __is_nt_destructible_safe_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nt_destructible_safe_272 {}
impl Clone for __is_nt_destructible_safe_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_destructible_273 {}
impl Clone for is_nothrow_destructible_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_destructible_274 {}
impl Clone for is_nothrow_destructible_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_destructible_275 {}
impl Clone for is_nothrow_destructible_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_276 {}
impl Clone for is_constructible_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_277 {}
impl Clone for is_constructible_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_278 {}
impl Clone for is_constructible_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_279 {}
impl Clone for is_constructible_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_280 {}
impl Clone for is_constructible_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_constructible_281 {}
impl Clone for is_constructible_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_default_constructible_282 {}
impl Clone for is_default_constructible_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_default_constructible_283 {}
impl Clone for is_default_constructible_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_284 {}
impl Clone for is_move_constructible_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_285 {}
impl Clone for is_move_constructible_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_286 {}
impl Clone for is_move_constructible_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_287 {}
impl Clone for is_move_constructible_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_constructible_288 {}
impl Clone for is_move_constructible_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_289 {}
impl Clone for is_nothrow_default_constructible_289 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_289 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_290 {}
impl Clone for is_nothrow_default_constructible_290 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_290 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_291 {}
impl Clone for is_nothrow_default_constructible_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_292 {}
impl Clone for is_nothrow_default_constructible_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_293 {}
impl Clone for is_nothrow_default_constructible_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_default_constructible_294 {}
impl Clone for is_nothrow_default_constructible_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_295 {}
impl Clone for is_nothrow_copy_constructible_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_296 {}
impl Clone for is_nothrow_copy_constructible_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_297 {}
impl Clone for is_nothrow_copy_constructible_297 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_297 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_298 {}
impl Clone for is_nothrow_copy_constructible_298 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_298 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_copy_constructible_299 {}
impl Clone for is_nothrow_copy_constructible_299 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_299 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_nothrow_move_constructible_300 {}
impl Clone for is_nothrow_move_constructible_300 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_300 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_301 {}
impl Clone for is_assignable_301 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_301 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_302 {}
impl Clone for is_assignable_302 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_302 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_303 {}
impl Clone for is_assignable_303 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_303 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_assignable_304 {}
impl Clone for is_assignable_304 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_304 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_move_assignable_305 {}
impl Clone for is_move_assignable_305 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_305 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___do_is_implicitly_default_constructible_impl {}
impl Clone for std___do_is_implicitly_default_constructible_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_is_implicitly_default_constructible_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_impl_306 {}
impl Clone for __is_implicitly_default_constructible_impl_306 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_306 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_impl_307 {}
impl Clone for __is_implicitly_default_constructible_impl_307 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_307 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_safe_308 {}
impl Clone for __is_implicitly_default_constructible_safe_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_safe_309 {}
impl Clone for __is_implicitly_default_constructible_safe_309 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_309 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_310 {}
impl Clone for __is_implicitly_default_constructible_310 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_310 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_implicitly_default_constructible_311 {}
impl Clone for __is_implicitly_default_constructible_311 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_311 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_trivially_destructible_312 {}
impl Clone for is_trivially_destructible_312 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_312 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_trivially_destructible_313 {}
impl Clone for is_trivially_destructible_313 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_313 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_trivially_destructible_314 {}
impl Clone for is_trivially_destructible_314 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_314 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_trivially_destructible_315 {}
impl Clone for is_trivially_destructible_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct is_convertible_316 {}
impl Clone for is_convertible_316 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_316 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nothrow_new_constructible_impl_317 {}
impl Clone for __is_nothrow_new_constructible_impl_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nothrow_new_constructible_impl_318 {}
impl Clone for __is_nothrow_new_constructible_impl_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nothrow_new_constructible_impl_319 {}
impl Clone for __is_nothrow_new_constructible_impl_319 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_319 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nothrow_new_constructible_impl_320 {}
impl Clone for __is_nothrow_new_constructible_impl_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nothrow_new_constructible_impl_321 {}
impl Clone for __is_nothrow_new_constructible_impl_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nothrow_new_constructible_impl_322 {}
impl Clone for __is_nothrow_new_constructible_impl_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_323 {}
impl Clone for remove_reference_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_324 {}
impl Clone for remove_reference_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_324 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_325 {}
impl Clone for remove_reference_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_326 {}
impl Clone for remove_reference_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_327 {}
impl Clone for remove_reference_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_328 {}
impl Clone for remove_reference_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_329 {}
impl Clone for remove_reference_329 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_329 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_330 {}
impl Clone for remove_reference_330 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_330 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_331 {}
impl Clone for remove_reference_331 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_331 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_332 {}
impl Clone for remove_reference_332 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_332 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_333 {}
impl Clone for remove_reference_333 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_333 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_334 {}
impl Clone for remove_reference_334 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_334 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_335 {}
impl Clone for remove_reference_335 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_335 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_336 {}
impl Clone for remove_reference_336 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_336 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_337 {}
impl Clone for remove_reference_337 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_337 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_338 {}
impl Clone for remove_reference_338 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_338 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_339 {}
impl Clone for remove_reference_339 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_339 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_340 {}
impl Clone for remove_reference_340 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_340 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_341 {}
impl Clone for remove_reference_341 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_341 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_342 {}
impl Clone for remove_reference_342 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_342 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_343 {}
impl Clone for remove_reference_343 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_343 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_344 {}
impl Clone for remove_reference_344 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_344 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_345 {}
impl Clone for remove_reference_345 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_345 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_346 {}
impl Clone for remove_reference_346 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_346 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_reference_347 {}
impl Clone for remove_reference_347 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_347 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct add_lvalue_reference_348 {}
impl Clone for add_lvalue_reference_348 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_lvalue_reference_348 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_349 {}
impl Clone for __cv_selector_349 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_349 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_350 {}
impl Clone for __cv_selector_350 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_350 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_351 {}
impl Clone for __cv_selector_351 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_351 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __cv_selector_352 {}
impl Clone for __cv_selector_352 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_352 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_353 {}
impl Clone for __match_cv_qualifiers_353 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_353 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_354 {}
impl Clone for __match_cv_qualifiers_354 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_354 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_355 {}
impl Clone for __match_cv_qualifiers_355 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_355 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_356 {}
impl Clone for __match_cv_qualifiers_356 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_356 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __match_cv_qualifiers_357 {}
impl Clone for __match_cv_qualifiers_357 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_357 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_358 {}
impl Clone for __make_unsigned_358 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_358 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_359 {}
impl Clone for __make_unsigned_359 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_359 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_360 {}
impl Clone for __make_unsigned_360 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_360 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_361 {}
impl Clone for __make_unsigned_selector_361 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_361 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_362 {}
impl Clone for __make_unsigned_selector_362 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_362 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_363 {}
impl Clone for __make_unsigned_selector_363 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_363 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_364 {}
impl Clone for __make_unsigned_selector_364 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_364 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_365 {}
impl Clone for __make_unsigned_selector_365 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_365 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_unsigned_selector_366 {}
impl Clone for __make_unsigned_selector_366 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_366 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___make_unsigned_selector_base {}
impl Clone for std___make_unsigned_selector_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___make_unsigned_selector_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_370 {}
impl Clone for _List_370 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_370 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_371 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_372 {}
impl Clone for _List_372 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_372 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_373 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_374 {}
impl Clone for _List_374 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_374 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_375 : std::cell::LazyCell< usize > = std::cell::LazyCell::new(|| unsafe { unimplemented!() } ) ;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_376 {}
impl Clone for _List_376 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_376 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_377 {}
impl Clone for _List_377 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_377 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _List_378 {}
impl Clone for _List_378 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_378 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_379 {}
impl Clone for __select_379 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_379 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_380 {}
impl Clone for __select_380 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_380 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_381 {}
impl Clone for __select_381 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_381 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_382 {}
impl Clone for __select_382 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_382 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __select_383 {}
impl Clone for __select_383 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_383 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_384 {}
impl Clone for __make_signed_384 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_384 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_385 {}
impl Clone for __make_signed_385 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_385 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_386 {}
impl Clone for __make_signed_selector_386 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_386 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_387 {}
impl Clone for __make_signed_selector_387 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_387 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_388 {}
impl Clone for __make_signed_selector_388 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_388 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_389 {}
impl Clone for __make_signed_selector_389 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_389 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __make_signed_selector_390 {}
impl Clone for __make_signed_selector_390 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_390 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct remove_pointer_391 {}
impl Clone for remove_pointer_391 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_391 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___aligned_storage_max_align_t {}
impl Clone for std___aligned_storage_max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___aligned_storage_max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct common_type_392 {}
impl Clone for common_type_392 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_392 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __success_type_393 {}
impl Clone for __success_type_393 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __success_type_393 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___failure_type {}
impl Clone for std___failure_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___failure_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___do_common_type_impl {}
impl Clone for std___do_common_type_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___do_common_type_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___invoke_memfun_ref {}
impl Clone for std___invoke_memfun_ref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memfun_ref {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___invoke_memfun_deref {}
impl Clone for std___invoke_memfun_deref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memfun_deref {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___invoke_memobj_ref {}
impl Clone for std___invoke_memobj_ref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memobj_ref {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___invoke_memobj_deref {}
impl Clone for std___invoke_memobj_deref {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_memobj_deref {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___invoke_other {}
impl Clone for std___invoke_other {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___invoke_other {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __result_of_success_394 {}
impl Clone for __result_of_success_394 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_success_394 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___result_of_memfun_ref_impl {}
impl Clone for std___result_of_memfun_ref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memfun_ref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___result_of_memfun_deref_impl {}
impl Clone for std___result_of_memfun_deref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memfun_deref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___result_of_memobj_ref_impl {}
impl Clone for std___result_of_memobj_ref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memobj_ref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___result_of_memobj_deref_impl {}
impl Clone for std___result_of_memobj_deref_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_memobj_deref_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __result_of_impl_395 {}
impl Clone for __result_of_impl_395 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_impl_395 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___result_of_other_impl {}
impl Clone for std___result_of_other_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___result_of_other_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __invoke_result_396 {}
impl Clone for __invoke_result_396 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __invoke_result_396 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___swappable_details___do_is_swappable_impl {}
impl Clone for std___swappable_details___do_is_swappable_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_details___do_is_swappable_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___swappable_details___do_is_nothrow_swappable_impl {}
impl Clone for std___swappable_details___do_is_nothrow_swappable_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_details___do_is_nothrow_swappable_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___swappable_with_details___do_is_swappable_with_impl {}
impl Clone for std___swappable_with_details___do_is_swappable_with_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_with_details___do_is_swappable_with_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___swappable_with_details___do_is_nothrow_swappable_with_impl {}
impl Clone for std___swappable_with_details___do_is_nothrow_swappable_with_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___swappable_with_details___do_is_nothrow_swappable_with_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_invocable_impl_397 {}
impl Clone for __is_invocable_impl_397 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_impl_397 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_invocable_398 {}
impl Clone for __is_invocable_398 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_398 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___nonesuchbase {}
impl Clone for std___nonesuchbase {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___nonesuchbase {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___nonesuchbase {}
impl std___nonesuchbase {}
impl std___nonesuchbase {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std___nonesuchbase,
    ) -> *mut std___nonesuchbase {
        unimplemented!()
    }
}
impl std___nonesuchbase {
    pub unsafe fn move_assign(&mut self, _a0: *mut std___nonesuchbase) -> *mut std___nonesuchbase {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___nonesuch {}
impl Clone for std___nonesuch {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___nonesuch {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __is_nothrow_new_constructible_399: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __is_nothrow_new_constructible_400: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __is_nothrow_new_constructible_401: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __is_nothrow_new_constructible_402: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __is_nothrow_new_constructible_403: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __is_nothrow_new_constructible_404: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_405 {}
impl Clone for __new_allocator_405 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_405 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_405 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_405) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_405 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_405,
    ) -> *mut __new_allocator_405 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_406 {}
impl Clone for __new_allocator_406 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_406 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_406 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_406) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_406 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_406,
    ) -> *mut __new_allocator_406 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_407 {}
impl Clone for __new_allocator_407 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_407 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_407 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_407 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_407) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_407 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_407,
    ) -> *mut __new_allocator_407 {
        unimplemented!()
    }
}
impl __new_allocator_407 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut i32 {
        unimplemented!()
    }
}
impl __new_allocator_407 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut i32, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_407 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_407 {
    pub unsafe fn construct_408(&mut self, mut __p: *mut i32, __args: *mut i32) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_409 {}
impl Clone for __new_allocator_409 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_409 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_409 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_409 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_409) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_409 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_409,
    ) -> *mut __new_allocator_409 {
        unimplemented!()
    }
}
impl __new_allocator_409 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut vector_10 {
        unimplemented!()
    }
}
impl __new_allocator_409 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut vector_10, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_409 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_409 {
    pub unsafe fn construct_410(&mut self, mut __p: *mut vector_10, __args: *const vector_10) {
        unimplemented!()
    }
}
impl __new_allocator_409 {
    pub unsafe fn construct_411(&mut self, mut __p: *mut vector_10, __args: *mut vector_10) {
        unimplemented!()
    }
}
impl __new_allocator_409 {
    pub unsafe fn destroy_412(&mut self, mut __p: *mut vector_10) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_413 {}
impl Clone for __new_allocator_413 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_413 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_413 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_413 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_413) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_413 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_413,
    ) -> *mut __new_allocator_413 {
        unimplemented!()
    }
}
impl __new_allocator_413 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut Explicit {
        unimplemented!()
    }
}
impl __new_allocator_413 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut Explicit, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_413 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_413 {
    pub unsafe fn construct_414(&mut self, mut __p: *mut Explicit, __args: *const Explicit) {
        unimplemented!()
    }
}
impl __new_allocator_413 {
    pub unsafe fn construct_415(&mut self, mut __p: *mut Explicit, __args: *mut Explicit) {
        unimplemented!()
    }
}
impl __new_allocator_413 {
    pub unsafe fn destroy_416(&mut self, mut __p: *mut Explicit) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __new_allocator_417 {}
impl Clone for __new_allocator_417 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_417 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn copy_from(_a0: *const __new_allocator_417) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __new_allocator_417,
    ) -> *mut __new_allocator_417 {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn allocate(
        &mut self,
        mut __n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut Buffer {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn deallocate(&mut self, mut __p: *mut Buffer, mut __n: u64) {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn construct_418(&mut self, mut __p: *mut Buffer, __args: *mut Buffer) {
        unimplemented!()
    }
}
impl __new_allocator_417 {
    pub unsafe fn destroy_419(&mut self, mut __p: *mut Buffer) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_input_iterator_tag {}
impl Clone for std_input_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_input_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_input_iterator_tag {}
impl std_input_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_output_iterator_tag {}
impl Clone for std_output_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_output_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_forward_iterator_tag {}
impl Clone for std_forward_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_forward_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_forward_iterator_tag {}
impl std_forward_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_bidirectional_iterator_tag {}
impl Clone for std_bidirectional_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_bidirectional_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_bidirectional_iterator_tag {}
impl std_bidirectional_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_random_access_iterator_tag {}
impl Clone for std_random_access_iterator_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_random_access_iterator_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_random_access_iterator_tag {}
impl std_random_access_iterator_tag {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_420 {}
impl Clone for iterator_420 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_420 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_421 {}
impl Clone for iterator_421 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_421 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_421 {}
impl iterator_421 {}
impl iterator_421 {}
impl iterator_421 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_421) -> *mut iterator_421 {
        unimplemented!()
    }
}
impl iterator_421 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_421) -> *mut iterator_421 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_422 {}
impl Clone for iterator_422 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_422 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_422 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_422) -> *mut iterator_422 {
        unimplemented!()
    }
}
impl iterator_422 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_422) -> *mut iterator_422 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_423 {}
impl Clone for iterator_423 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_423 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_423 {
    pub unsafe fn copy_assign(&mut self, _a0: *const iterator_423) -> *mut iterator_423 {
        unimplemented!()
    }
}
impl iterator_423 {
    pub unsafe fn move_assign(&mut self, _a0: *mut iterator_423) -> *mut iterator_423 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_424 {}
impl Clone for iterator_traits_424 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_424 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_425 {}
impl Clone for iterator_traits_425 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_425 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_426 {}
impl Clone for iterator_traits_426 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_426 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_427 {}
impl Clone for iterator_traits_427 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_427 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_428 {}
impl Clone for iterator_traits_428 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_428 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_429 {}
impl Clone for iterator_traits_429 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_429 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_430 {}
impl Clone for iterator_traits_430 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_430 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_431 {}
impl Clone for iterator_traits_431 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_431 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_432 {}
impl Clone for iterator_traits_432 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_432 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_433 {}
impl Clone for iterator_traits_433 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_433 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct iterator_traits_434 {}
impl Clone for iterator_traits_434 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_434 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_435 {}
impl Clone for __iterator_traits_435 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_435 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_436 {}
impl Clone for __iterator_traits_436 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_436 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __iterator_traits_437 {}
impl Clone for __iterator_traits_437 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_437 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_element_438 {}
impl Clone for tuple_element_438 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_438 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_element_439 {}
impl Clone for tuple_element_439 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_439 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_in_place_t {}
impl Clone for std_in_place_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_in_place_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_in_place_t {}
impl std_in_place_t {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Nth_type_440 {}
impl Clone for _Nth_type_440 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_440 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Nth_type_441 {}
impl Clone for _Nth_type_441 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_441 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Swallow_assign {}
impl Clone for std__Swallow_assign {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Swallow_assign {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_piecewise_construct_t {}
impl Clone for std_piecewise_construct_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_piecewise_construct_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_piecewise_construct_t {}
impl std_piecewise_construct_t {}
pub static mut piecewise_construct_442: std::cell::LazyCell<std_piecewise_construct_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_443 {}
impl Clone for tuple_443 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_443 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_443 {}
impl tuple_443 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct tuple_444 {}
impl Clone for tuple_444 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_444 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_444 {
    pub unsafe fn copy_from(_a0: *const tuple_444) -> Self {
        unimplemented!()
    }
}
impl tuple_444 {
    pub unsafe fn move_from(_a0: *mut tuple_444) -> Self {
        unimplemented!()
    }
}
impl tuple_444 {
    pub unsafe fn copy_assign(&mut self, __in: *const tuple_444) -> *mut tuple_444 {
        unimplemented!()
    }
}
impl tuple_444 {
    pub unsafe fn move_assign(&mut self, __in: *mut tuple_444) -> *mut tuple_444 {
        unimplemented!()
    }
}
impl tuple_444 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___true_type {}
impl Clone for std___true_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___true_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___false_type {}
impl Clone for std___false_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___false_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_445 {}
impl Clone for __are_same_445 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_445 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_446 = u32;
pub const anon_446___value: anon_446 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_447 {}
impl Clone for __are_same_447 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_447 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_448 = u32;
pub const anon_448___value: anon_448 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_449 {}
impl Clone for __are_same_449 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_449 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_450 = u32;
pub const anon_450___value: anon_450 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_451 {}
impl Clone for __are_same_451 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_451 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_452 = u32;
pub const anon_452___value: anon_452 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_453 {}
impl Clone for __are_same_453 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_453 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_454 = u32;
pub const anon_454___value: anon_454 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __are_same_455 {}
impl Clone for __are_same_455 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_455 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_456 = u32;
pub const anon_456___value: anon_456 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_457 {}
impl Clone for __is_integer_457 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_457 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_458 = u32;
pub const anon_458___value: anon_458 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_459 {}
impl Clone for __is_integer_459 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_459 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_460 = u32;
pub const anon_460___value: anon_460 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_461 {}
impl Clone for __is_integer_461 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_461 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_462 = u32;
pub const anon_462___value: anon_462 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_463 {}
impl Clone for __is_integer_463 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_463 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_464 = u32;
pub const anon_464___value: anon_464 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_465 {}
impl Clone for __is_integer_465 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_465 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_466 = u32;
pub const anon_466___value: anon_466 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_byte_467 {}
impl Clone for __is_byte_467 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_byte_467 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_468 = u32;
pub const anon_468___value: anon_468 = 0;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_nonvolatile_trivially_copyable_469 {}
impl Clone for __is_nonvolatile_trivially_copyable_469 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_469 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_470 = u32;
pub const anon_470___value: anon_470 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_471 {}
impl Clone for __memcpyable_471 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_471 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __memcpyable_integer_472 {}
impl Clone for __memcpyable_integer_472 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_472 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_473 {}
impl Clone for __is_integer_nonstrict_473 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_473 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_474 = u32;
pub const anon_474___width: anon_474 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_integer_nonstrict_475 {}
impl Clone for __is_integer_nonstrict_475 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_475 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_476 = u32;
pub const anon_476___width: anon_476 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_477 {}
impl Clone for __numeric_traits_integer_477 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_477 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_long____is_signed_478: std::cell::LazyCell<bool> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____digits_479: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_long____max_480: std::cell::LazyCell<i64> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_integer_481 {}
impl Clone for __numeric_traits_integer_481 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_481 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_482:
    std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____digits_483: std::cell::LazyCell<
    i32,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
pub static mut __gnu_cxx___numeric_traits_integer_unsigned_long____max_484: std::cell::LazyCell<
    u64,
> = std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_485 {}
impl Clone for __numeric_traits_floating_485 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_485 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_486 {}
impl Clone for __numeric_traits_floating_486 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_486 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_floating_487 {}
impl Clone for __numeric_traits_floating_487 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_487 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __numeric_traits_488 {}
impl Clone for __numeric_traits_488 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_488 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_489 {}
impl Clone for __ptr_traits_ptr_to_489 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_489 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_490 {}
impl Clone for __ptr_traits_ptr_to_490 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_490 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_491 {}
impl Clone for __ptr_traits_ptr_to_491 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_491 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __ptr_traits_ptr_to_492 {}
impl Clone for __ptr_traits_ptr_to_492 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_492 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_493 {}
impl Clone for pointer_traits_493 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_493 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_494 {}
impl Clone for pointer_traits_494 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_494 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_495 {}
impl Clone for pointer_traits_495 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_495 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct pointer_traits_496 {}
impl Clone for pointer_traits_496 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_496 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reverse_iterator_497 {}
impl Clone for reverse_iterator_497 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_497 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_497 {
    pub unsafe fn copy_from(__x: *const reverse_iterator_497) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_497 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reverse_iterator_497,
    ) -> *mut reverse_iterator_497 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct reverse_iterator_498 {}
impl Clone for reverse_iterator_498 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_498 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_498 {
    pub unsafe fn copy_from(__x: *const reverse_iterator_498) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_498 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const reverse_iterator_498,
    ) -> *mut reverse_iterator_498 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_499 {}
impl Clone for __normal_iterator_499 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_499 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_499 {
    pub unsafe fn new_1(__i: *const *mut i32) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_499 {
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        unimplemented!()
    }
}
impl __normal_iterator_499 {
    pub unsafe fn operator_sub(&self, mut __n: i64) -> __normal_iterator_499 {
        unimplemented!()
    }
}
impl __normal_iterator_499 {
    pub unsafe fn base(&self) -> *const *mut i32 {
        unimplemented!()
    }
}
impl __normal_iterator_499 {}
impl __normal_iterator_499 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_500 {}
impl Clone for __normal_iterator_500 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_500 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_500 {
    pub unsafe fn new_1(__i: *const *const i32) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_500 {
    pub unsafe fn base(&self) -> *const *const i32 {
        unimplemented!()
    }
}
impl __normal_iterator_500 {}
impl __normal_iterator_500 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_501 {}
impl Clone for __normal_iterator_501 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_501 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_501 {
    pub unsafe fn new_1(__i: *const *mut vector_10) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_501 {
    pub unsafe fn base(&self) -> *const *mut vector_10 {
        unimplemented!()
    }
}
impl __normal_iterator_501 {}
impl __normal_iterator_501 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_502 {}
impl Clone for __normal_iterator_502 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_502 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_502 {
    pub unsafe fn new_1(__i: *const *const vector_10) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_502 {
    pub unsafe fn base(&self) -> *const *const vector_10 {
        unimplemented!()
    }
}
impl __normal_iterator_502 {}
impl __normal_iterator_502 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_503 {}
impl Clone for __normal_iterator_503 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_503 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_503 {
    pub unsafe fn new_1(__i: *const *mut Explicit) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_503 {
    pub unsafe fn operator_deref(&self) -> *mut Explicit {
        unimplemented!()
    }
}
impl __normal_iterator_503 {
    pub unsafe fn operator_sub(&self, mut __n: i64) -> __normal_iterator_503 {
        unimplemented!()
    }
}
impl __normal_iterator_503 {
    pub unsafe fn base(&self) -> *const *mut Explicit {
        unimplemented!()
    }
}
impl __normal_iterator_503 {}
impl __normal_iterator_503 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_504 {}
impl Clone for __normal_iterator_504 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_504 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_504 {
    pub unsafe fn new_1(__i: *const *const Explicit) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_504 {
    pub unsafe fn base(&self) -> *const *const Explicit {
        unimplemented!()
    }
}
impl __normal_iterator_504 {}
impl __normal_iterator_504 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_505 {}
impl Clone for __normal_iterator_505 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_505 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_505 {
    pub unsafe fn new_1(__i: *const *mut Buffer) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_505 {
    pub unsafe fn operator_deref(&self) -> *mut Buffer {
        unimplemented!()
    }
}
impl __normal_iterator_505 {
    pub unsafe fn operator_sub(&self, mut __n: i64) -> __normal_iterator_505 {
        unimplemented!()
    }
}
impl __normal_iterator_505 {
    pub unsafe fn base(&self) -> *const *mut Buffer {
        unimplemented!()
    }
}
impl __normal_iterator_505 {}
impl __normal_iterator_505 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __normal_iterator_506 {}
impl Clone for __normal_iterator_506 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_506 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_506 {
    pub unsafe fn new_1(__i: *const *const Buffer) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_506 {
    pub unsafe fn base(&self) -> *const *const Buffer {
        unimplemented!()
    }
}
impl __normal_iterator_506 {}
impl __normal_iterator_506 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___ops__Iter_less_iter {}
impl Clone for __gnu_cxx___ops__Iter_less_iter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_less_iter {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_iter {}
impl __gnu_cxx___ops__Iter_less_iter {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___ops__Iter_less_val {}
impl Clone for __gnu_cxx___ops__Iter_less_val {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_less_val {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_less_val {}
impl __gnu_cxx___ops__Iter_less_val {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___ops__Val_less_iter {}
impl Clone for __gnu_cxx___ops__Val_less_iter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Val_less_iter {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Val_less_iter {}
impl __gnu_cxx___ops__Val_less_iter {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___ops__Iter_equal_to_iter {}
impl Clone for __gnu_cxx___ops__Iter_equal_to_iter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_equal_to_iter {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_equal_to_iter {}
impl __gnu_cxx___ops__Iter_equal_to_iter {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___ops__Iter_equal_to_val {}
impl Clone for __gnu_cxx___ops__Iter_equal_to_val {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___ops__Iter_equal_to_val {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___ops__Iter_equal_to_val {}
impl __gnu_cxx___ops__Iter_equal_to_val {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___allocator_traits_base {}
impl Clone for std___allocator_traits_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___allocator_traits_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_move_insertable_508 {}
impl Clone for __is_move_insertable_508 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_508 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_move_insertable_509 {}
impl Clone for __is_move_insertable_509 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_509 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_move_insertable_510 {}
impl Clone for __is_move_insertable_510 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_510 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_move_insertable_511 {}
impl Clone for __is_move_insertable_511 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_511 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_512 {}
impl Clone for __alloc_traits_512 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_512 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_513 {}
impl Clone for rebind_513 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_513 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __alloc_traits_512 {
    pub unsafe fn _S_select_on_copy(__a: *const allocator_3) -> allocator_3 {
        unimplemented!()
    }
}
impl __alloc_traits_512 {
    pub unsafe fn _S_propagate_on_move_assign() -> bool {
        unimplemented!()
    }
}
impl __alloc_traits_512 {
    pub unsafe fn _S_always_equal() -> bool {
        unimplemented!()
    }
}
impl __alloc_traits_512 {
    pub unsafe fn _S_nothrow_move() -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_514 {}
impl Clone for __alloc_traits_514 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_514 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_515 {}
impl Clone for rebind_515 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_515 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __alloc_traits_514 {
    pub unsafe fn _S_propagate_on_move_assign() -> bool {
        unimplemented!()
    }
}
impl __alloc_traits_514 {
    pub unsafe fn _S_always_equal() -> bool {
        unimplemented!()
    }
}
impl __alloc_traits_514 {
    pub unsafe fn _S_nothrow_move() -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_516 {}
impl Clone for __alloc_traits_516 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_516 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_517 {}
impl Clone for rebind_517 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_517 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __alloc_traits_518 {}
impl Clone for __alloc_traits_518 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_518 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct rebind_519 {}
impl Clone for rebind_519 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_519 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _UninitDestroyGuard_520 {
    pub _M_first: *mut i32,
    pub _M_cur: *mut *mut i32,
}
impl Clone for _UninitDestroyGuard_520 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_520 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_520 {
    pub unsafe fn new(__first: *mut *mut i32) -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_520 {
    pub unsafe fn release(&mut self) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_bitwise_relocatable_521 {}
impl Clone for __is_bitwise_relocatable_521 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_521 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_bitwise_relocatable_522 {}
impl Clone for __is_bitwise_relocatable_522 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_522 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_bitwise_relocatable_523 {}
impl Clone for __is_bitwise_relocatable_523 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_523 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_bitwise_relocatable_524 {}
impl Clone for __is_bitwise_relocatable_524 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_524 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___erased_type {}
impl Clone for std___erased_type {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___erased_type {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_allocator_arg_t {}
impl Clone for std_allocator_arg_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_allocator_arg_t {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_allocator_arg_t {}
impl std_allocator_arg_t {}
pub static mut allocator_arg_525: std::cell::LazyCell<std_allocator_arg_t> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___uses_alloc_base {}
impl Clone for std___uses_alloc_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___uses_alloc_base {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std___uses_alloc0 {
    pub _M_a: std___uses_alloc0__Sink,
}
impl Clone for std___uses_alloc0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___uses_alloc0 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_empty_non_tuple_526 {}
impl Clone for __is_empty_non_tuple_526 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_526 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __is_empty_non_tuple_527 {}
impl Clone for __is_empty_non_tuple_527 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_527 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Head_base_528 {
    pub _M_head_impl: default_delete_529,
}
impl Clone for _Head_base_528 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_528 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_528 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_528 {
    pub unsafe fn copy_from(_a0: *const _Head_base_528) -> Self {
        unimplemented!()
    }
}
impl _Head_base_528 {
    pub unsafe fn move_from(_a0: *mut _Head_base_528) -> Self {
        unimplemented!()
    }
}
impl _Head_base_528 {
    pub unsafe fn _M_head_12(__b: *mut _Head_base_528) -> *mut default_delete_529 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Head_base_530 {
    pub _M_head_impl: *mut i32,
}
impl Clone for _Head_base_530 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_530 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_530 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_530 {
    pub unsafe fn copy_from(_a0: *const _Head_base_530) -> Self {
        unimplemented!()
    }
}
impl _Head_base_530 {
    pub unsafe fn move_from(_a0: *mut _Head_base_530) -> Self {
        unimplemented!()
    }
}
impl _Head_base_530 {
    pub unsafe fn _M_head_12(__b: *mut _Head_base_530) -> *mut *mut i32 {
        unimplemented!()
    }
}
impl _Head_base_530 {
    pub unsafe fn _M_head_13(__b: *const _Head_base_530) -> *const *mut i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Tuple_impl_531 {}
impl Clone for _Tuple_impl_531 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_531 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_531 {
    pub unsafe fn _M_head_1(__t: *mut _Tuple_impl_531) -> *mut *mut i32 {
        unimplemented!()
    }
}
impl _Tuple_impl_531 {
    pub unsafe fn _M_head_2(__t: *const _Tuple_impl_531) -> *const *mut i32 {
        unimplemented!()
    }
}
impl _Tuple_impl_531 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_531 {
    pub unsafe fn copy_from(_a0: *const _Tuple_impl_531) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_531 {
    pub unsafe fn move_from(_a0: *mut _Tuple_impl_531) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Tuple_impl_532 {}
impl Clone for _Tuple_impl_532 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_532 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_532 {
    pub unsafe fn _M_head_1(__t: *mut _Tuple_impl_532) -> *mut default_delete_529 {
        unimplemented!()
    }
}
impl _Tuple_impl_532 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_532 {
    pub unsafe fn copy_from(_a0: *const _Tuple_impl_532) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_532 {
    pub unsafe fn move_from(__in: *mut _Tuple_impl_532) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _TupleConstraints_533 {}
impl Clone for _TupleConstraints_533 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _TupleConstraints_533 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _TupleConstraints_533 {
    pub unsafe fn __is_implicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
impl _TupleConstraints_533 {
    pub unsafe fn __is_explicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct binary_function_534 {}
impl Clone for binary_function_534 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_534 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_535 {}
impl Clone for greater_535 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_535 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_536 {}
impl Clone for less_536 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_536 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct greater_equal_537 {}
impl Clone for greater_equal_537 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_537 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct less_equal_538 {}
impl Clone for less_equal_538 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_538 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_539 {}
impl Clone for __hash_base_539 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_539 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_540 {}
impl Clone for __hash_base_540 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_540 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_541 {}
impl Clone for __hash_base_541 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_541 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_542 {}
impl Clone for __hash_base_542 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_542 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_543 {}
impl Clone for __hash_base_543 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_543 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_544 {}
impl Clone for __hash_base_544 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_544 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_545 {}
impl Clone for __hash_base_545 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_545 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_546 {}
impl Clone for __hash_base_546 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_546 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_547 {}
impl Clone for __hash_base_547 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_547 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_548 {}
impl Clone for __hash_base_548 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_548 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_549 {}
impl Clone for __hash_base_549 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_549 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_550 {}
impl Clone for __hash_base_550 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_550 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_551 {}
impl Clone for __hash_base_551 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_551 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_552 {}
impl Clone for __hash_base_552 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_552 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_553 {}
impl Clone for __hash_base_553 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_553 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_554 {}
impl Clone for __hash_base_554 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_554 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_555 {}
impl Clone for __hash_base_555 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_555 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_556 {}
impl Clone for __hash_base_556 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_556 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_557 {}
impl Clone for __hash_base_557 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_557 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_558 {}
impl Clone for __hash_base_558 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_558 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __hash_base_559 {}
impl Clone for __hash_base_559 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_559 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct hash_560 {}
impl Clone for hash_560 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_560 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Hash_impl {}
impl Clone for std__Hash_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Hash_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash(
        mut __ptr: *const ::libc::c_void,
        mut __clength: usize,
        mut __seed: Option<usize>,
    ) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_561(__val: *const f32) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub unsafe fn hash_562(__val: *const f64) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct default_delete_529 {}
impl Clone for default_delete_529 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for default_delete_529 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl default_delete_529 {
    pub unsafe fn operator_call(&self, mut __ptr: *mut i32) {
        unimplemented!()
    }
}
impl default_delete_529 {}
impl default_delete_529 {}
impl default_delete_529 {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const default_delete_529,
    ) -> *mut default_delete_529 {
        unimplemented!()
    }
}
impl default_delete_529 {
    pub unsafe fn move_assign(&mut self, _a0: *mut default_delete_529) -> *mut default_delete_529 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __uniq_ptr_impl_563 {}
impl Clone for __uniq_ptr_impl_563 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_impl_563 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Ptr_564 {}
impl Clone for _Ptr_564 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_564 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_563 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_563 {
    pub unsafe fn move_from(__u: *mut __uniq_ptr_impl_563) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_563 {
    pub unsafe fn move_assign(
        &mut self,
        __u: *mut __uniq_ptr_impl_563,
    ) -> *mut __uniq_ptr_impl_563 {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_563 {
    pub unsafe fn _M_ptr_6(&mut self) -> *mut *mut i32 {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_563 {
    pub unsafe fn _M_ptr_7(&self) -> *mut i32 {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_563 {
    pub unsafe fn _M_deleter_8(&mut self) -> *mut default_delete_529 {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_563 {
    pub unsafe fn reset(&mut self, mut __p: *mut i32) {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_563 {
    pub unsafe fn release(&mut self) -> *mut i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __uniq_ptr_data_565 {}
impl Clone for __uniq_ptr_data_565 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_data_565 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_565 {
    pub unsafe fn move_from(_a0: *mut __uniq_ptr_data_565) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_565 {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __uniq_ptr_data_565,
    ) -> *mut __uniq_ptr_data_565 {
        unimplemented!()
    }
}
impl __uniq_ptr_data_565 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct unique_ptr_566 {}
impl Clone for unique_ptr_566 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for unique_ptr_566 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl unique_ptr_566 {
    pub unsafe fn move_from(_a0: *mut unique_ptr_566) -> Self {
        unimplemented!()
    }
}
impl unique_ptr_566 {
    pub unsafe fn move_assign(&mut self, _a0: *mut unique_ptr_566) -> *mut unique_ptr_566 {
        unimplemented!()
    }
}
impl unique_ptr_566 {
    pub unsafe fn operator_deref(&self) -> *mut i32 {
        unimplemented!()
    }
}
impl unique_ptr_566 {
    pub unsafe fn get(&self) -> *mut i32 {
        unimplemented!()
    }
}
impl unique_ptr_566 {
    pub unsafe fn get_deleter_17(&mut self) -> *mut default_delete_529 {
        unimplemented!()
    }
}
impl unique_ptr_566 {
    pub unsafe fn reset(&mut self, mut __p: *mut i32) {
        unimplemented!()
    }
}
impl unique_ptr_566 {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: anon_567,
}
impl Clone for __mbstate_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __mbstate_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_567 {
    pub __wch: u32,
    pub __wchb: [libc::c_char; 4],
}
impl Clone for anon_567 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_567 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13],
    pub __ctype_b: *const u16,
    pub __ctype_tolower: *const i32,
    pub __ctype_toupper: *const i32,
    pub __names: [*const libc::c_char; 13],
}
impl Clone for __locale_struct {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __locale_struct {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_type_info {}
impl Clone for std_type_info {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_type_info {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_type_info {
    pub unsafe fn name(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl std_type_info {
    pub unsafe fn operator_eq(&self, __arg: *const std_type_info) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __fsid_t {
    pub __val: [i32; 2],
}
impl Clone for __fsid_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __fsid_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}
impl Clone for timespec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timespec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct sched_param {
    pub sched_priority: i32,
}
impl Clone for sched_param {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for sched_param {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct cpu_set_t {
    pub __bits: [u64; 16],
}
impl Clone for cpu_set_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for cpu_set_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}
impl Clone for timeval {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timeval {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct timex {
    pub modes: u32,
    pub offset: i64,
    pub freq: i64,
    pub maxerror: i64,
    pub esterror: i64,
    pub status: i32,
    pub constant: i64,
    pub precision: i64,
    pub tolerance: i64,
    pub time: timeval,
    pub tick: i64,
    pub ppsfreq: i64,
    pub jitter: i64,
    pub shift: i32,
    pub stabil: i64,
    pub jitcnt: i64,
    pub calcnt: i64,
    pub errcnt: i64,
    pub stbcnt: i64,
    pub tai: i32,
    pub anon_568: i32,
    pub anon_569: i32,
    pub anon_570: i32,
    pub anon_571: i32,
    pub anon_572: i32,
    pub anon_573: i32,
    pub anon_574: i32,
    pub anon_575: i32,
    pub anon_576: i32,
    pub anon_577: i32,
    pub anon_578: i32,
}
impl Clone for timex {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for timex {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const libc::c_char,
}
impl Clone for tm {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tm {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
impl Clone for itimerspec {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for itimerspec {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __atomic_wide_counter {
    pub __value64: u64,
    pub __value32: anon_579,
}
impl Clone for __atomic_wide_counter {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __atomic_wide_counter {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct anon_579 {
    pub __low: u32,
    pub __high: u32,
}
impl Clone for anon_579 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_579 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
impl Clone for __pthread_internal_list {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_list {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
}
impl Clone for __pthread_internal_slist {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_internal_slist {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_mutex_s {
    pub __lock: i32,
    pub __count: u32,
    pub __owner: i32,
    pub __nusers: u32,
    pub __kind: i32,
    pub __spins: i16,
    pub __elision: i16,
    pub __list: __pthread_internal_list,
}
impl Clone for __pthread_mutex_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_mutex_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: u32,
    pub __writers: u32,
    pub __wrphase_futex: u32,
    pub __writers_futex: u32,
    pub __pad3: u32,
    pub __pad4: u32,
    pub __cur_writer: i32,
    pub __shared: i32,
    pub __rwelision: i8,
    pub __pad1: [u8; 7],
    pub __pad2: u64,
    pub __flags: u32,
}
impl Clone for __pthread_rwlock_arch_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_rwlock_arch_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [u32; 2],
    pub __g_size: [u32; 2],
    pub __g1_orig_size: u32,
    pub __wrefs: u32,
    pub __g_signals: [u32; 2],
}
impl Clone for __pthread_cond_s {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cond_s {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __once_flag {
    pub __data: i32,
}
impl Clone for __once_flag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __once_flag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: i32,
}
impl Clone for pthread_mutexattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutexattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: i32,
}
impl Clone for pthread_condattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_condattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: i64,
}
impl Clone for pthread_attr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_attr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: i64,
}
impl Clone for pthread_mutex_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_mutex_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: i64,
}
impl Clone for pthread_cond_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_cond_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [libc::c_char; 56],
    pub __align: i64,
}
impl Clone for pthread_rwlock_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlock_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_rwlockattr_t {
    pub __size: [libc::c_char; 8],
    pub __align: i64,
}
impl Clone for pthread_rwlockattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_rwlockattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrier_t {
    pub __size: [libc::c_char; 32],
    pub __align: i64,
}
impl Clone for pthread_barrier_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrier_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct pthread_barrierattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: i32,
}
impl Clone for pthread_barrierattr_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pthread_barrierattr_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
impl Clone for __sigset_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __sigset_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: [i64; 8],
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
impl Clone for __jmp_buf_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __jmp_buf_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_580 = u32;
pub const anon_580_PTHREAD_MUTEX_TIMED_NP: anon_580 = 0;
pub const anon_580_PTHREAD_MUTEX_RECURSIVE_NP: anon_580 = 1;
pub const anon_580_PTHREAD_MUTEX_ERRORCHECK_NP: anon_580 = 2;
pub const anon_580_PTHREAD_MUTEX_ADAPTIVE_NP: anon_580 = 3;
pub const anon_580_PTHREAD_MUTEX_NORMAL: anon_580 = 0;
pub const anon_580_PTHREAD_MUTEX_RECURSIVE: anon_580 = 1;
pub const anon_580_PTHREAD_MUTEX_ERRORCHECK: anon_580 = 2;
pub const anon_580_PTHREAD_MUTEX_DEFAULT: anon_580 = 0;
pub const anon_580_PTHREAD_MUTEX_FAST_NP: anon_580 = 0;
pub type anon_581 = u32;
pub const anon_581_PTHREAD_MUTEX_STALLED: anon_581 = 0;
pub const anon_581_PTHREAD_MUTEX_STALLED_NP: anon_581 = 0;
pub const anon_581_PTHREAD_MUTEX_ROBUST: anon_581 = 1;
pub const anon_581_PTHREAD_MUTEX_ROBUST_NP: anon_581 = 1;
pub type anon_582 = u32;
pub const anon_582_PTHREAD_RWLOCK_PREFER_READER_NP: anon_582 = 0;
pub const anon_582_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_582 = 1;
pub const anon_582_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_582 = 2;
pub const anon_582_PTHREAD_RWLOCK_DEFAULT_NP: anon_582 = 0;
#[derive(VaArg, FnPtrArg)]
pub struct _pthread_cleanup_buffer {
    pub __routine: Option<unsafe fn(*mut ::libc::c_void)>,
    pub __arg: *mut ::libc::c_void,
    pub __canceltype: i32,
    pub __prev: *mut _pthread_cleanup_buffer,
}
impl Clone for _pthread_cleanup_buffer {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _pthread_cleanup_buffer {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_583 = u32;
pub const anon_583_PTHREAD_CANCEL_DEFERRED: anon_583 = 0;
pub const anon_583_PTHREAD_CANCEL_ASYNCHRONOUS: anon_583 = 1;
#[derive(VaArg, FnPtrArg)]
pub struct __cancel_jmp_buf_tag {
    pub __cancel_jmp_buf: [i64; 8],
    pub __mask_was_saved: i32,
}
impl Clone for __cancel_jmp_buf_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cancel_jmp_buf_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_unwind_buf_t {
    pub __cancel_jmp_buf: [__cancel_jmp_buf_tag; 1],
    pub __pad: [*mut ::libc::c_void; 4],
}
impl Clone for __pthread_unwind_buf_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_unwind_buf_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct __pthread_cleanup_frame {
    pub __cancel_routine: Option<unsafe fn(*mut ::libc::c_void)>,
    pub __cancel_arg: *mut ::libc::c_void,
    pub __do_it: i32,
    pub __cancel_type: i32,
}
impl Clone for __pthread_cleanup_frame {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cleanup_frame {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __pthread_cleanup_class {}
impl Clone for __pthread_cleanup_class {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pthread_cleanup_class {
    fn default() -> Self {
        unimplemented!()
    }
}
pub static mut __libc_single_threaded_584: std::cell::LazyCell<libc::c_char> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___exception_ptr_exception_ptr {}
impl Clone for std___exception_ptr_exception_ptr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___exception_ptr_exception_ptr {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn copy_from(__other: *const std___exception_ptr_exception_ptr) -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn move_from(__o: *mut std___exception_ptr_exception_ptr) -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn copy_assign(
        &mut self,
        __other: *const std___exception_ptr_exception_ptr,
    ) -> *mut std___exception_ptr_exception_ptr {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn move_assign(
        &mut self,
        __o: *mut std___exception_ptr_exception_ptr,
    ) -> *mut std___exception_ptr_exception_ptr {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn swap(&mut self, __other: *mut std___exception_ptr_exception_ptr) {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_nested_exception {}
impl Clone for std_nested_exception {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_nested_exception {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_nested_exception {
    pub unsafe fn copy_from(_a0: *const std_nested_exception) -> Self {
        unimplemented!()
    }
}
impl std_nested_exception {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std_nested_exception,
    ) -> *mut std_nested_exception {
        unimplemented!()
    }
}
pub type __gnu_cxx__Lock_policy = u32;
pub const __gnu_cxx__Lock_policy__S_single: __gnu_cxx__Lock_policy = 0;
pub const __gnu_cxx__Lock_policy__S_mutex: __gnu_cxx__Lock_policy = 1;
pub const __gnu_cxx__Lock_policy__S_atomic: __gnu_cxx__Lock_policy = 2;
pub static mut __default_lock_policy_585: std::cell::LazyCell<__gnu_cxx__Lock_policy> =
    std::cell::LazyCell::new(|| unsafe { unimplemented!() });
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___concurrence_lock_error {}
impl Clone for __gnu_cxx___concurrence_lock_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___concurrence_lock_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {
    unsafe fn what(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {}
impl __gnu_cxx___concurrence_lock_error {}
impl __gnu_cxx___concurrence_lock_error {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_cxx___concurrence_lock_error,
    ) -> *mut __gnu_cxx___concurrence_lock_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_cxx___concurrence_lock_error,
    ) -> *mut __gnu_cxx___concurrence_lock_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___concurrence_unlock_error {}
impl Clone for __gnu_cxx___concurrence_unlock_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___concurrence_unlock_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {
    unsafe fn what(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {}
impl __gnu_cxx___concurrence_unlock_error {}
impl __gnu_cxx___concurrence_unlock_error {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_cxx___concurrence_unlock_error,
    ) -> *mut __gnu_cxx___concurrence_unlock_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_cxx___concurrence_unlock_error,
    ) -> *mut __gnu_cxx___concurrence_unlock_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___concurrence_broadcast_error {}
impl Clone for __gnu_cxx___concurrence_broadcast_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___concurrence_broadcast_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {
    unsafe fn what(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {}
impl __gnu_cxx___concurrence_broadcast_error {}
impl __gnu_cxx___concurrence_broadcast_error {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_cxx___concurrence_broadcast_error,
    ) -> *mut __gnu_cxx___concurrence_broadcast_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_cxx___concurrence_broadcast_error,
    ) -> *mut __gnu_cxx___concurrence_broadcast_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___concurrence_wait_error {}
impl Clone for __gnu_cxx___concurrence_wait_error {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___concurrence_wait_error {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {
    unsafe fn what(&self) -> *const libc::c_char {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {}
impl __gnu_cxx___concurrence_wait_error {}
impl __gnu_cxx___concurrence_wait_error {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const __gnu_cxx___concurrence_wait_error,
    ) -> *mut __gnu_cxx___concurrence_wait_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut __gnu_cxx___concurrence_wait_error,
    ) -> *mut __gnu_cxx___concurrence_wait_error {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___mutex {}
impl Clone for __gnu_cxx___mutex {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___mutex {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___mutex {
    pub unsafe fn lock(&mut self) {
        unimplemented!()
    }
}
impl __gnu_cxx___mutex {
    pub unsafe fn unlock(&mut self) {
        unimplemented!()
    }
}
impl __gnu_cxx___mutex {
    pub unsafe fn gthread_mutex(&mut self) -> *mut pthread_mutex_t {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___recursive_mutex {}
impl Clone for __gnu_cxx___recursive_mutex {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___recursive_mutex {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___recursive_mutex {
    pub unsafe fn gthread_recursive_mutex(&mut self) -> *mut pthread_mutex_t {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___scoped_lock {}
impl Clone for __gnu_cxx___scoped_lock {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___scoped_lock {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __gnu_cxx___scoped_lock {
    pub unsafe fn new(__name: *mut __gnu_cxx___mutex) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __gnu_cxx___cond {}
impl Clone for __gnu_cxx___cond {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __gnu_cxx___cond {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_bad_weak_ptr {}
impl Clone for std_bad_weak_ptr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_bad_weak_ptr {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_bad_weak_ptr {}
impl std_bad_weak_ptr {
    pub unsafe fn copy_assign(&mut self, _a0: *const std_bad_weak_ptr) -> *mut std_bad_weak_ptr {
        unimplemented!()
    }
}
impl std_bad_weak_ptr {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Mutex_base_586 {}
impl Clone for _Mutex_base_586 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_586 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_587 = u32;
pub const anon_587__S_need_barriers: anon_587 = 1;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Mutex_base_588 {}
impl Clone for _Mutex_base_588 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_588 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Mutex_base_588 {}
impl _Mutex_base_588 {}
impl _Mutex_base_588 {
    pub unsafe fn copy_assign(&mut self, _a0: *const _Mutex_base_588) -> *mut _Mutex_base_588 {
        unimplemented!()
    }
}
impl _Mutex_base_588 {
    pub unsafe fn move_assign(&mut self, _a0: *mut _Mutex_base_588) -> *mut _Mutex_base_588 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Mutex_base_589 {}
impl Clone for _Mutex_base_589 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_589 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_590 = u32;
pub const anon_590__S_need_barriers: anon_590 = 0;
impl _Mutex_base_589 {}
impl _Mutex_base_589 {}
impl _Mutex_base_589 {
    pub unsafe fn copy_assign(&mut self, _a0: *const _Mutex_base_589) -> *mut _Mutex_base_589 {
        unimplemented!()
    }
}
impl _Mutex_base_589 {
    pub unsafe fn move_assign(&mut self, _a0: *mut _Mutex_base_589) -> *mut _Mutex_base_589 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Sp_counted_ptr_591 {}
impl Clone for _Sp_counted_ptr_591 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_591 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Sp_counted_ptr_592 {}
impl Clone for _Sp_counted_ptr_592 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_592 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Sp_counted_ptr_593 {}
impl Clone for _Sp_counted_ptr_593 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_593 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Sp_make_shared_tag {}
impl Clone for std__Sp_make_shared_tag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Sp_make_shared_tag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Sp_alloc_shared_tag_594 {
    pub _M_a: *const allocator_0,
}
impl Clone for _Sp_alloc_shared_tag_594 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_alloc_shared_tag_594 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std___sp_array_delete {}
impl Clone for std___sp_array_delete {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___sp_array_delete {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct _Sp_owner_less_595 {}
impl Clone for _Sp_owner_less_595 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_owner_less_595 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_memory_order = i32;
pub const std_memory_order_memory_order_relaxed: std_memory_order = 0;
pub const std_memory_order_memory_order_consume: std_memory_order = 1;
pub const std_memory_order_memory_order_acquire: std_memory_order = 2;
pub const std_memory_order_memory_order_release: std_memory_order = 3;
pub const std_memory_order_memory_order_acq_rel: std_memory_order = 4;
pub const std_memory_order_memory_order_seq_cst: std_memory_order = 5;
pub type std___memory_order_modifier = u32;
pub const std___memory_order_modifier___memory_order_mask: std___memory_order_modifier = 65535;
pub const std___memory_order_modifier___memory_order_modifier_mask: std___memory_order_modifier =
    4294901760;
pub const std___memory_order_modifier___memory_order_hle_acquire: std___memory_order_modifier =
    65536;
pub const std___memory_order_modifier___memory_order_hle_release: std___memory_order_modifier =
    131072;
#[derive(VaArg, FnPtrArg)]
pub struct std___atomic_flag_base {
    pub _M_i: bool,
}
impl Clone for std___atomic_flag_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std___atomic_flag_base {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std___atomic_flag_base {}
impl std___atomic_flag_base {}
impl std___atomic_flag_base {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std___atomic_flag_base,
    ) -> *mut std___atomic_flag_base {
        unimplemented!()
    }
}
impl std___atomic_flag_base {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std___atomic_flag_base,
    ) -> *mut std___atomic_flag_base {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std_atomic_flag {}
impl Clone for std_atomic_flag {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_atomic_flag {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Sp_locker {}
impl Clone for std__Sp_locker {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Sp_locker {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_pointer_safety = i32;
pub const std_pointer_safety_relaxed: std_pointer_safety = 0;
pub const std_pointer_safety_preferred: std_pointer_safety = 1;
pub const std_pointer_safety_strict: std_pointer_safety = 2;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __pstl_execution_sequenced_policy {}
impl Clone for __pstl_execution_sequenced_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pstl_execution_sequenced_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __pstl_execution_parallel_policy {}
impl Clone for __pstl_execution_parallel_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pstl_execution_parallel_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __pstl_execution_parallel_unsequenced_policy {}
impl Clone for __pstl_execution_parallel_unsequenced_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pstl_execution_parallel_unsequenced_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct __pstl_execution_unsequenced_policy {}
impl Clone for __pstl_execution_unsequenced_policy {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __pstl_execution_unsequenced_policy {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_596 {}
impl Clone for initializer_list_596 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_596 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_596 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl initializer_list_596 {
    pub unsafe fn begin(&self) -> *const bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_597 {}
impl Clone for initializer_list_597 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_597 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_597 {}
impl initializer_list_597 {}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct initializer_list_598 {}
impl Clone for initializer_list_598 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_598 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_598 {}
impl initializer_list_598 {}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_599 {
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_599 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_599 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub _M_start: *mut i32,
    pub _M_finish: *mut i32,
    pub _M_end_of_storage: *mut i32,
}
impl Clone for std__Vector_base_int__std_allocator_int____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_int__std_allocator_int____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_int__std_allocator_int____Vector_impl_data,
    ) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub unsafe fn _M_copy_data(
        &mut self,
        __x: *const std__Vector_base_int__std_allocator_int____Vector_impl_data,
    ) {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub unsafe fn _M_swap_data(
        &mut self,
        __x: *mut std__Vector_base_int__std_allocator_int____Vector_impl_data,
    ) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Vector_base_int__std_allocator_int____Vector_impl {}
impl Clone for std__Vector_base_int__std_allocator_int____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_int__std_allocator_int____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub unsafe fn new_2(__a: *const allocator_3) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_int__std_allocator_int____Vector_impl,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_599 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_3 {
        unimplemented!()
    }
}
impl _Vector_base_599 {
    pub unsafe fn _M_get_Tp_allocator_2(&self) -> *const allocator_3 {
        unimplemented!()
    }
}
impl _Vector_base_599 {
    pub unsafe fn get_allocator(&self) -> allocator_3 {
        unimplemented!()
    }
}
impl _Vector_base_599 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_599 {
    pub unsafe fn new_2(__a: *const allocator_3) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_599 {
    pub unsafe fn new_3(mut __n: usize, __a: *const allocator_3) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_599 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_599) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_599 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut i32 {
        unimplemented!()
    }
}
impl _Vector_base_599 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut i32, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_600 {
    pub _M_impl: std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl,
}
impl Clone for _Vector_base_600 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_600 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    pub _M_start: *mut vector_10,
    pub _M_finish: *mut vector_10,
    pub _M_end_of_storage: *mut vector_10,
}
impl Clone for std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default
    for std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data
{
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    pub unsafe fn move_from(
        __x : *mut std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data,
    ) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    pub unsafe fn _M_copy_data(
        &mut self,
        __x : *const std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data,
    ) {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data {
    pub unsafe fn _M_swap_data(
        &mut self,
        __x : *mut std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl_data,
    ) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl {}
impl Clone for std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl {
    pub unsafe fn new_2(__a: *const allocator_4) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_std_vector_int___std_allocator_std_vector_int_____Vector_impl,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_600 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_4 {
        unimplemented!()
    }
}
impl _Vector_base_600 {
    pub unsafe fn _M_get_Tp_allocator_2(&self) -> *const allocator_4 {
        unimplemented!()
    }
}
impl _Vector_base_600 {
    pub unsafe fn get_allocator(&self) -> allocator_4 {
        unimplemented!()
    }
}
impl _Vector_base_600 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_600 {
    pub unsafe fn new_2(__a: *const allocator_4) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_600 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_600) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_600 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut vector_10 {
        unimplemented!()
    }
}
impl _Vector_base_600 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut vector_10, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_601 {
    pub _M_impl: std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl,
}
impl Clone for _Vector_base_601 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_601 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl_data {
    pub _M_start: *mut Explicit,
    pub _M_finish: *mut Explicit,
    pub _M_end_of_storage: *mut Explicit,
}
impl Clone for std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl_data {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl_data {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl_data,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl {}
impl Clone for std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_Explicit__std_allocator_Explicit____Vector_impl,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_601 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_5 {
        unimplemented!()
    }
}
impl _Vector_base_601 {
    pub unsafe fn _M_get_Tp_allocator_2(&self) -> *const allocator_5 {
        unimplemented!()
    }
}
impl _Vector_base_601 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_601 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_601) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_601 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut Explicit {
        unimplemented!()
    }
}
impl _Vector_base_601 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut Explicit, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct _Vector_base_602 {
    pub _M_impl: std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl,
}
impl Clone for _Vector_base_602 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_602 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl_data {
    pub _M_start: *mut Buffer,
    pub _M_finish: *mut Buffer,
    pub _M_end_of_storage: *mut Buffer,
}
impl Clone for std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl_data {
    pub unsafe fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl_data {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl_data,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl {}
impl Clone for std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl {
    pub unsafe fn move_from(
        __x: *mut std__Vector_base_Buffer__std_allocator_Buffer____Vector_impl,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_602 {
    pub unsafe fn _M_get_Tp_allocator_1(&mut self) -> *mut allocator_6 {
        unimplemented!()
    }
}
impl _Vector_base_602 {
    pub unsafe fn _M_get_Tp_allocator_2(&self) -> *const allocator_6 {
        unimplemented!()
    }
}
impl _Vector_base_602 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_602 {
    pub unsafe fn move_from(_a0: *mut _Vector_base_602) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_602 {
    pub unsafe fn _M_allocate(&mut self, mut __n: usize) -> *mut Buffer {
        unimplemented!()
    }
}
impl _Vector_base_602 {
    pub unsafe fn _M_deallocate(&mut self, mut __p: *mut Buffer, mut __n: usize) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_10 {}
impl Clone for vector_10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_10 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_vector_int___Guard_alloc {
    pub _M_storage: *mut i32,
    pub _M_len: u64,
    pub _M_vect: *mut _Vector_base_599,
}
impl Clone for std_vector_int___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_int___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_int___Guard_alloc {
    pub unsafe fn new(mut __s: *mut i32, mut __l: u64, __vect: *mut _Vector_base_599) -> Self {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn new_2(__a: *const allocator_3) -> Self {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn new_3(
        mut __n: u64,
        __value: *const i32,
        __a: Option<*const allocator_3>,
    ) -> Self {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn copy_from(__x: *const vector_10) -> Self {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn move_from(_a0: *mut vector_10) -> Self {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_10) -> *mut vector_10 {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_10) -> *mut vector_10 {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn begin_26(&mut self) -> __normal_iterator_499 {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn begin_27(&self) -> __normal_iterator_500 {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn end_28(&mut self) -> __normal_iterator_499 {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn end_29(&self) -> __normal_iterator_500 {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn empty(&self) -> bool {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut i32 {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn back_53(&mut self) -> *mut i32 {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn push_back_58(&mut self, __x: *mut i32) {
        unimplemented!()
    }
}
impl vector_10 {
    pub unsafe fn emplace_back_603(&mut self, __args: *mut i32) -> *mut i32 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_604 {}
impl Clone for vector_604 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_604 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_vector_std_vector_int____Guard_alloc {
    pub _M_storage: *mut vector_10,
    pub _M_len: u64,
    pub _M_vect: *mut _Vector_base_600,
}
impl Clone for std_vector_std_vector_int____Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_std_vector_int____Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_std_vector_int____Guard_alloc {
    pub unsafe fn new(
        mut __s: *mut vector_10,
        mut __l: u64,
        __vect: *mut _Vector_base_600,
    ) -> Self {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn new_2(__a: *const allocator_4) -> Self {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn copy_from(__x: *const vector_604) -> Self {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn move_from(_a0: *mut vector_604) -> Self {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_604) -> *mut vector_604 {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_604) -> *mut vector_604 {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn begin_26(&mut self) -> __normal_iterator_501 {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn begin_27(&self) -> __normal_iterator_502 {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn end_28(&mut self) -> __normal_iterator_501 {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn end_29(&self) -> __normal_iterator_502 {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn empty(&self) -> bool {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut vector_10 {
        unimplemented!()
    }
}
impl vector_604 {
    pub unsafe fn push_back_57(&mut self, __x: *const vector_10) {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_605 {}
impl Clone for vector_605 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_605 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_vector_Explicit___Guard_alloc {
    pub _M_storage: *mut Explicit,
    pub _M_len: u64,
    pub _M_vect: *mut _Vector_base_601,
}
impl Clone for std_vector_Explicit___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_Explicit___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_Explicit___Guard_alloc {
    pub unsafe fn new(mut __s: *mut Explicit, mut __l: u64, __vect: *mut _Vector_base_601) -> Self {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn copy_from(__x: *const vector_605) -> Self {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn move_from(_a0: *mut vector_605) -> Self {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_605) -> *mut vector_605 {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_605) -> *mut vector_605 {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn begin_26(&mut self) -> __normal_iterator_503 {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn begin_27(&self) -> __normal_iterator_504 {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn end_28(&mut self) -> __normal_iterator_503 {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn end_29(&self) -> __normal_iterator_504 {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn empty(&self) -> bool {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut Explicit {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn back_53(&mut self) -> *mut Explicit {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn push_back_57(&mut self, __x: *const Explicit) {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn push_back_58(&mut self, __x: *mut Explicit) {
        unimplemented!()
    }
}
impl vector_605 {
    pub unsafe fn emplace_back_606(&mut self, __args: *mut Explicit) -> *mut Explicit {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct vector_607 {}
impl Clone for vector_607 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_607 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std_vector_Buffer___Guard_alloc {
    pub _M_storage: *mut Buffer,
    pub _M_len: u64,
    pub _M_vect: *mut _Vector_base_602,
}
impl Clone for std_vector_Buffer___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_Buffer___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_Buffer___Guard_alloc {
    pub unsafe fn new(mut __s: *mut Buffer, mut __l: u64, __vect: *mut _Vector_base_602) -> Self {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn copy_from(__x: *const vector_607) -> Self {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn move_from(_a0: *mut vector_607) -> Self {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn copy_assign(&mut self, __x: *const vector_607) -> *mut vector_607 {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn move_assign(&mut self, __x: *mut vector_607) -> *mut vector_607 {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn begin_26(&mut self) -> __normal_iterator_505 {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn begin_27(&self) -> __normal_iterator_506 {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn end_28(&mut self) -> __normal_iterator_505 {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn end_29(&self) -> __normal_iterator_506 {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn empty(&self) -> bool {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn operator_index_46(&mut self, mut __n: u64) -> *mut Buffer {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn back_53(&mut self) -> *mut Buffer {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn push_back_58(&mut self, __x: *mut Buffer) {
        unimplemented!()
    }
}
impl vector_607 {
    pub unsafe fn emplace_back_608(&mut self, __args: *mut Buffer) -> *mut Buffer {
        unimplemented!()
    }
}
pub type anon_609 = u32;
pub const anon_609__S_word_bit: anon_609 = 64;
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Bit_reference {}
impl Clone for std__Bit_reference {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_reference {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn copy_from(_a0: *const std__Bit_reference) -> Self {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn operator_assign_5(&mut self, mut __x: bool) -> *mut std__Bit_reference {
        unimplemented!()
    }
}
impl std__Bit_reference {
    pub unsafe fn copy_assign(
        &mut self,
        __x: *const std__Bit_reference,
    ) -> *mut std__Bit_reference {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg)]
pub struct std__Bit_iterator_base {
    pub _M_p: *mut u64,
    pub _M_offset: u32,
}
impl Clone for std__Bit_iterator_base {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_iterator_base {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_assume_normalized(&self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn new(mut __x: *mut u64, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_bump_up(&mut self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_bump_down(&mut self) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn _M_incr(&mut self, mut __i: i64) {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std__Bit_iterator_base,
    ) -> *mut std__Bit_iterator_base {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std__Bit_iterator_base,
    ) -> *mut std__Bit_iterator_base {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Bit_iterator {}
impl Clone for std__Bit_iterator {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_iterator {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn new_1(mut __x: *mut u64, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_deref(&self) -> std__Bit_reference {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_add_assign(&mut self, mut __i: i64) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn operator_sub_assign(&mut self, mut __i: i64) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {}
impl std__Bit_iterator {}
impl std__Bit_iterator {
    pub unsafe fn copy_assign(&mut self, _a0: *const std__Bit_iterator) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub unsafe fn move_assign(&mut self, _a0: *mut std__Bit_iterator) -> *mut std__Bit_iterator {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy)]
pub struct std__Bit_const_iterator {}
impl Clone for std__Bit_const_iterator {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Bit_const_iterator {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn operator_deref(&self) -> bool {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn operator_add_assign(&mut self, mut __i: i64) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn operator_sub_assign(&mut self, mut __i: i64) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {
    pub unsafe fn copy_assign(
        &mut self,
        _a0: *const std__Bit_const_iterator,
    ) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut std__Bit_const_iterator,
    ) -> *mut std__Bit_const_iterator {
        unimplemented!()
    }
}
pub type std_byte = u8;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Inner {
    pub x: i32,
}
#[repr(C)]
#[derive(Clone, VaArg, DestructorUnsafe, FnPtrArg, Default)]
pub struct Explicit {
    pub v: i32,
    pub inner: Inner,
    pub arr: [i32; 2],
}
impl Explicit {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self {
            v: v,
            inner: Inner { x: ((v) * (10)) },
            arr: [v, ((v) + (1))],
        };
        this
    }
    pub unsafe fn destructor(&mut self) {}
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Implicit {
    pub v: i32,
    pub inner: Inner,
    pub arr: [i32; 2],
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, MoveCtorUnsafe, FnPtrArg, Default)]
pub struct DefaultCopyUserMove {
    pub v: i32,
}
impl DefaultCopyUserMove {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn move_from(o: *mut DefaultCopyUserMove) -> Self {
        let mut this = Self { v: (*o).v };
        (*o).v = 0;
        this
    }
    pub unsafe fn move_assign(&mut self, o: *mut DefaultCopyUserMove) -> *mut DefaultCopyUserMove {
        self.v = (*o).v;
        (*o).v = 0;
        return &mut (*(self as *mut DefaultCopyUserMove));
    }
}
#[repr(C)]
#[derive(VaArg, MoveCtorUnsafe, FnPtrArg, Default)]
pub struct UserCopyDefaultMove {
    pub v: i32,
}
impl UserCopyDefaultMove {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self { v: v };
        this
    }
    pub unsafe fn copy_from(o: *const UserCopyDefaultMove) -> Self {
        let mut this = Self {
            v: (((*o).v) + (100)),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut UserCopyDefaultMove) -> Self {
        let mut this = Self { v: (*_a0).v };
        this
    }
    pub unsafe fn copy_assign(
        &mut self,
        o: *const UserCopyDefaultMove,
    ) -> *mut UserCopyDefaultMove {
        self.v = (((*o).v) + (100));
        return &mut (*(self as *mut UserCopyDefaultMove));
    }
    pub unsafe fn move_assign(
        &mut self,
        _a0: *mut UserCopyDefaultMove,
    ) -> *mut UserCopyDefaultMove {
        self.v = (*_a0).v;
        return &mut (*(self as *mut UserCopyDefaultMove));
    }
}
impl Clone for UserCopyDefaultMove {
    fn clone(&self) -> Self {
        unsafe { UserCopyDefaultMove::copy_from(self as *const UserCopyDefaultMove) }
    }
}
#[repr(C)]
#[derive(MoveCtorUnsafe, FnPtrArg, Default)]
pub struct Buffer {
    pub data: vector_10,
    pub rows: vector_604,
    pub n: i32,
    pub arr: [i32; 2],
}
impl Buffer {
    pub unsafe fn new(mut n: i32) -> Self {
        let mut this = Self {
            data: vector_10::new_3({ (n as u64) }, { &n }, None),
            rows: <vector_604>::default(),
            n: n,
            arr: [n, ((n) + (1))],
        };
        (unsafe {
            let ___x: *const vector_10 = &this.data;
            vector_604::push_back_57(&mut this.rows, ___x)
        });
        this
    }
    pub unsafe fn move_from(_a0: *mut Buffer) -> Self {
        let mut this = Self {
            data: vector_10::move_from({ &mut (*_a0).data }),
            rows: vector_604::move_from({ &mut (*_a0).rows }),
            n: (*_a0).n,
            arr: std::array::from_fn::<_, 2, _>(|__i: usize| (*_a0).arr[(__i)]),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Buffer) -> *mut Buffer {
        self.data = ((*_a0).data).clone();
        self.rows = ((*_a0).rows).clone();
        self.n = (*_a0).n;
        (unsafe {
            let _arg0: *mut ::libc::c_void =
                ((&mut self.arr as *mut [i32; 2]) as *mut ::libc::c_void);
            let _arg1: *const ::libc::c_void =
                ((&mut (*_a0).arr as *mut [i32; 2]) as *const ::libc::c_void);
            __builtin_memcpy_610(_arg0, _arg1, 8_usize)
        });
        return &mut (*(self as *mut Buffer));
    }
}
#[repr(C)]
#[derive(MoveCtorUnsafe, FnPtrArg, Default)]
pub struct Owner {
    pub data: vector_10,
    pub n: i32,
    pub arr: [i32; 2],
    pub p: unique_ptr_566,
}
impl Owner {
    pub unsafe fn move_from(_a0: *mut Owner) -> Self {
        let mut this = Self {
            data: vector_10::move_from({ &mut (*_a0).data }),
            n: (*_a0).n,
            arr: std::array::from_fn::<_, 2, _>(|__i: usize| (*_a0).arr[(__i)]),
            p: unique_ptr_566::move_from({ &mut (*_a0).p }),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Owner) -> *mut Owner {
        self.data = ((*_a0).data).clone();
        self.n = (*_a0).n;
        (unsafe {
            let _arg0: *mut ::libc::c_void =
                ((&mut self.arr as *mut [i32; 2]) as *mut ::libc::c_void);
            let _arg1: *const ::libc::c_void =
                ((&mut (*_a0).arr as *mut [i32; 2]) as *const ::libc::c_void);
            __builtin_memcpy_610(_arg0, _arg1, 8_usize)
        });
        self.p = ((*_a0).p).clone();
        return &mut (*(self as *mut Owner));
    }
}
#[repr(C)]
#[derive(MoveCtorUnsafe, DestructorUnsafe, FnPtrArg, Default)]
pub struct Holder {
    pub inner: Inner,
    pub e: Explicit,
    pub p: unique_ptr_566,
}
impl Holder {
    pub unsafe fn new(mut v: i32) -> Self {
        let mut this = Self {
            inner: Inner { x: v },
            e: Explicit::new({ v }),
            p: unique_ptr_566::new(),
        };
        this
    }
    pub unsafe fn move_from(_a0: *mut Holder) -> Self {
        let mut this = Self {
            inner: (*_a0).inner,
            e: (*_a0).e.clone(),
            p: unique_ptr_566::move_from({ &mut (*_a0).p }),
        };
        this
    }
    pub unsafe fn move_assign(&mut self, _a0: *mut Holder) -> *mut Holder {
        self.inner = (*_a0).inner;
        self.e = ((*_a0).e).clone();
        self.p = ((*_a0).p).clone();
        return &mut (*(self as *mut Holder));
    }
}
impl Holder {
    pub unsafe fn destructor(&mut self) {
        Explicit::destructor(&mut self.e);
    }
}
pub unsafe fn same_611(a: *const Explicit, b: *const Explicit) -> bool {
    return (((((*a).v) == ((*b).v)) && (((*a).inner.x) == ((*b).inner.x)))
        && (((*a).arr[(0) as usize]) == ((*b).arr[(0) as usize])))
        && (((*a).arr[(1) as usize]) == ((*b).arr[(1) as usize]));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut a: Explicit = Explicit::new({ 1 });
    let _dtor_a = ScopedDestructorUnsafe::new(&raw mut a, Explicit::destructor);
    let mut b: Explicit = a.clone();
    let _dtor_b = ScopedDestructorUnsafe::new(&raw mut b, Explicit::destructor);
    let mut c: Explicit = a.clone();
    let _dtor_c = ScopedDestructorUnsafe::new(&raw mut c, Explicit::destructor);
    let mut d: Explicit = a.clone();
    let _dtor_d = ScopedDestructorUnsafe::new(&raw mut d, Explicit::destructor);
    (if ((unsafe { same_611(&b, &a) }) && (unsafe { same_611(&c, &a) }))
        && (unsafe { same_611(&d, &a) })
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"same(b, a) && same(c, a) && same(d, a)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut e: Explicit = Explicit::new({ 2 });
    let _dtor_e = ScopedDestructorUnsafe::new(&raw mut e, Explicit::destructor);
    let mut f: Explicit = Explicit::new({ 3 });
    let _dtor_f = ScopedDestructorUnsafe::new(&raw mut f, Explicit::destructor);
    e = (b).clone();
    f = (c).clone();
    (if (unsafe { same_611(&e, &b) }) && (unsafe { same_611(&f, &c) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"same(e, b) && same(f, c)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut g: Explicit = Explicit::new({ 4 });
    let _dtor_g = ScopedDestructorUnsafe::new(&raw mut g, Explicit::destructor);
    g = {
        e = (f).clone();
        (e).clone()
    };
    (if (unsafe { same_611(&g, &f) }) && (unsafe { same_611(&e, &f) }) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"same(g, f) && same(e, f)".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut i: Implicit = Implicit {
        v: 5,
        inner: Inner { x: 50 },
        arr: [5, 6],
    };
    let mut j: Implicit = i;
    let mut k: Implicit = i;
    (if (((j.v) == (5)) && ((j.inner.x) == (50))) && ((j.arr[(1) as usize]) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"j.v == 5 && j.inner.x == 50 && j.arr[1] == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((i.v) == (5)) && ((k.v) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"i.v == 5 && k.v == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut l: Implicit = Implicit {
        v: 0,
        inner: Inner { x: 0 },
        arr: [0, 0],
    };
    l = j;
    (if (((l.v) == (5)) && ((l.inner.x) == (50))) && ((l.arr[(0) as usize]) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"l.v == 5 && l.inner.x == 50 && l.arr[0] == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut vec_: vector_605 = <vector_605>::default();
    (unsafe { vector_605::push_back_57(&mut vec_, &b) });
    (unsafe {
        let mut ___x: Explicit = Explicit::new({ 9 });
        vector_605::push_back_58(&mut vec_, &mut ___x)
    });
    (if (((*(unsafe { vector_605::operator_index_46(&mut vec_, 0_u64) })).v) == (1))
        && (((*(unsafe { vector_605::operator_index_46(&mut vec_, 1_u64) })).v) == (9))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"vec[0].v == 1 && vec[1].v == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut m: DefaultCopyUserMove = DefaultCopyUserMove::new({ 7 });
    let mut m1: DefaultCopyUserMove = m;
    let mut m2: DefaultCopyUserMove = DefaultCopyUserMove::move_from({ &mut m });
    (if (((m1.v) == (7)) && ((m2.v) == (7))) && ((m.v) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"m1.v == 7 && m2.v == 7 && m.v == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut m3: DefaultCopyUserMove = DefaultCopyUserMove::new({ 1 });
    let mut m4: DefaultCopyUserMove = DefaultCopyUserMove::new({ 1 });
    m3 = m1;
    (unsafe { DefaultCopyUserMove::move_assign(&mut m4, &mut m1) });
    (if (((m3.v) == (7)) && ((m4.v) == (7))) && ((m1.v) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"m3.v == 7 && m4.v == 7 && m1.v == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut u: UserCopyDefaultMove = UserCopyDefaultMove::new({ 8 });
    let mut u1: UserCopyDefaultMove = UserCopyDefaultMove::copy_from({ &u });
    let mut u2: UserCopyDefaultMove = UserCopyDefaultMove::move_from({ &mut u });
    (if (((u1.v) == (108)) && ((u2.v) == (8))) && ((u.v) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"u1.v == 108 && u2.v == 8 && u.v == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut u3: UserCopyDefaultMove = UserCopyDefaultMove::new({ 1 });
    let mut u4: UserCopyDefaultMove = UserCopyDefaultMove::new({ 1 });
    (unsafe { UserCopyDefaultMove::copy_assign(&mut u3, &u2) });
    (unsafe { UserCopyDefaultMove::move_assign(&mut u4, &mut u2) });
    (if ((u3.v) == (108)) && ((u4.v) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"u3.v == 108 && u4.v == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p: Buffer = Buffer::new({ 3 });
    let mut q: Buffer = Buffer::move_from({ &mut p });
    (if ((((q.n) == (3)) && ((unsafe { vector_10::size(&q.data) }) == (3_u64)))
        && ((*(unsafe { vector_10::operator_index_46(&mut q.data, 2_u64) })) == (3)))
        && (unsafe { vector_10::empty(&p.data) })
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"q.n == 3 && q.data.size() == 3 && q.data[2] == 3 && p.data.empty()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut r: Buffer = Buffer::new({ 1 });
    (unsafe { Buffer::move_assign(&mut r, &mut q) });
    (if ((((r.n) == (3)) && ((unsafe { vector_10::size(&r.data) }) == (3_u64)))
        && ((r.arr[(1) as usize]) == (4)))
        && (unsafe { vector_10::empty(&q.data) })
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"r.n == 3 && r.data.size() == 3 && r.arr[1] == 4 && q.data.empty()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((unsafe { vector_604::size(&r.rows) }) == (1_u64))
        && ((unsafe {
            vector_10::size(&(*(unsafe { vector_604::operator_index_46(&mut r.rows, 0_u64) })))
        }) == (3_u64)))
        && (unsafe { vector_604::empty(&q.rows) })
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"r.rows.size() == 1 && r.rows[0].size() == 3 && q.rows.empty()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut bufs: vector_607 = <vector_607>::default();
    (unsafe { vector_607::push_back_58(&mut bufs, &mut r) });
    (unsafe {
        let ___args: *mut Buffer =
            &mut (*(unsafe { vector_607::operator_index_46(&mut bufs, 0_u64) }));
        vector_607::emplace_back_608(&mut bufs, ___args)
    });
    (if ((((*(unsafe { vector_607::operator_index_46(&mut bufs, 1_u64) })).n) == (3))
        && ((unsafe {
            vector_10::size(&(*(unsafe { vector_607::operator_index_46(&mut bufs, 1_u64) })).data)
        }) == (3_u64)))
        && (unsafe {
            vector_10::empty(&(*(unsafe { vector_607::operator_index_46(&mut bufs, 0_u64) })).data)
        })
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"bufs[1].n == 3 && bufs[1].data.size() == 3 && bufs[0].data.empty()".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut o1: Owner = <Owner>::default();
    (unsafe {
        let mut ___x: i32 = 5;
        vector_10::push_back_58(&mut o1.data, &mut ___x)
    });
    o1.n = 5;
    o1.arr[(0) as usize] = 5;
    o1.arr[(1) as usize] = 6;
    (unsafe { unique_ptr_566::reset(&mut o1.p, (Box::leak(Box::new(7)) as *mut i32)) });
    let mut o2: Owner = Owner::move_from({ &mut o1 });
    (if ((((o2.n) == (5)) && ((unsafe { vector_10::size(&o2.data) }) == (1_u64)))
        && ((o2.arr[(1) as usize]) == (6)))
        && ((*(unsafe { unique_ptr_566::operator_deref(&o2.p) })) == (7))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"o2.n == 5 && o2.data.size() == 1 && o2.arr[1] == 6 && *o2.p == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { vector_10::empty(&o1.data) })
        && ((unsafe { unique_ptr_566::get(&o1.p) }).is_null())
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"o1.data.empty() && o1.p.get() == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut o3: Owner = <Owner>::default();
    (unsafe { Owner::move_assign(&mut o3, &mut o2) });
    (if ((((o3.n) == (5))
        && ((*(unsafe { vector_10::operator_index_46(&mut o3.data, 0_u64) })) == (5)))
        && ((o3.arr[(0) as usize]) == (5)))
        && ((*(unsafe { unique_ptr_566::operator_deref(&o3.p) })) == (7))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"o3.n == 5 && o3.data[0] == 5 && o3.arr[0] == 5 && *o3.p == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (unsafe { vector_10::empty(&o2.data) })
        && ((unsafe { unique_ptr_566::get(&o2.p) }).is_null())
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"o2.data.empty() && o2.p.get() == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut h1: Holder = Holder::new({ 4 });
    let _dtor_h1 = ScopedDestructorUnsafe::new(&raw mut h1, Holder::destructor);
    (unsafe { unique_ptr_566::reset(&mut h1.p, (Box::leak(Box::new(9)) as *mut i32)) });
    let mut h2: Holder = Holder::move_from({ &mut h1 });
    let _dtor_h2 = ScopedDestructorUnsafe::new(&raw mut h2, Holder::destructor);
    (if ((((h2.inner.x) == (4)) && ((h2.e.v) == (4)))
        && ((*(unsafe { unique_ptr_566::operator_deref(&h2.p) })) == (9)))
        && ((unsafe { unique_ptr_566::get(&h1.p) }).is_null())
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"h2.inner.x == 4 && h2.e.v == 4 && *h2.p == 9 && h1.p.get() == nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut h3: Holder = Holder::new({ 1 });
    let _dtor_h3 = ScopedDestructorUnsafe::new(&raw mut h3, Holder::destructor);
    (unsafe { Holder::move_assign(&mut h3, &mut h2) });
    (if ((((h3.inner.x) == (4)) && ((h3.e.arr[(1) as usize]) == (5)))
        && ((*(unsafe { unique_ptr_566::operator_deref(&h3.p) })) == (9)))
        && ((unsafe { unique_ptr_566::get(&h2.p) }).is_null())
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_612(
                c"h3.inner.x == 4 && h3.e.arr[1] == 5 && *h3.p == 9 && h2.p.get() == nullptr"
                    .as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct __locale_data;
#[derive(Clone, Copy, Default, VaArg, FnPtrArg)]
pub struct std___uses_alloc0__Sink;
pub unsafe fn __builtin_memcpy_610(
    mut _a0: *mut ::libc::c_void,
    mut _a1: *const ::libc::c_void,
    mut _a2: usize,
) -> *mut ::libc::c_void {
    unimplemented!()
}
pub unsafe fn __assert_fail_612(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
