extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
    pub fn copy_from(__a: Ptr<allocator_1>) -> Self {
        unimplemented!()
    }
}
impl allocator_1 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_1>) -> Ptr<allocator_1> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
    pub fn copy_from(__a: Ptr<allocator_2>) -> Self {
        unimplemented!()
    }
}
impl allocator_2 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_2>) -> Ptr<allocator_2> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_3 {
    pub fn copy_from(__a: Ptr<allocator_3>) -> Self {
        unimplemented!()
    }
}
impl allocator_3 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_3>) -> Ptr<allocator_3> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_4 {}
impl Clone for allocator_traits_4 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_4 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_4 {
    pub fn allocate_1(__a: Ptr<allocator_3>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_4 {
    pub fn deallocate(__a: Ptr<allocator_3>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_4 {
    pub fn max_size(__a: Ptr<allocator_3>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_4 {
    pub fn select_on_container_copy_construction(__rhs: Ptr<allocator_3>) -> allocator_3 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
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
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub fn copy_from(_a0: Ptr<std_exception>) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub fn copy_assign(&self, _a0: Ptr<std_exception>) -> Ptr<std_exception> {
        unimplemented!()
    }
}
impl std_exception {
    pub fn move_from(_a0: Ptr<std_exception>) -> Self {
        unimplemented!()
    }
}
impl std_exception {
    pub fn move_assign(&self, _a0: Ptr<std_exception>) -> Ptr<std_exception> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
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
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub fn copy_from(_a0: Ptr<std_bad_alloc>) -> Self {
        unimplemented!()
    }
}
impl std_bad_alloc {
    pub fn copy_assign(&self, _a0: Ptr<std_bad_alloc>) -> Ptr<std_bad_alloc> {
        unimplemented!()
    }
}
pub type std_align_val_t = usize;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
thread_local!(
    pub static nothrow_5: Value<std_nothrow_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_6 {}
impl Clone for integral_constant_6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_6 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__true__value_7: Value<bool> = unimplemented!();
);
impl integral_constant_6 {}
impl integral_constant_6 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_8 {}
impl Clone for integral_constant_8 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_8 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__false__value_9: Value<bool> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_10 {}
impl Clone for integral_constant_10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_10 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_11 {}
impl Clone for integral_constant_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_11 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_12 {}
impl Clone for enable_if_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_12 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_13 {}
impl Clone for enable_if_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_13 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_14 {}
impl Clone for enable_if_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_15 {}
impl Clone for enable_if_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_15 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_16 {}
impl Clone for __conditional_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_17 {}
impl Clone for __conditional_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_17 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_18 {}
impl Clone for __type_identity_18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_18 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_19 {}
impl Clone for __type_identity_19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_19 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_20 {}
impl Clone for __type_identity_20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_20 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_21 {}
impl Clone for __type_identity_21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_21 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_22 {}
impl Clone for __type_identity_22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_22 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_23 {}
impl Clone for __type_identity_23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_23 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_24 {}
impl Clone for __type_identity_24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_24 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__25 {}
impl Clone for __or__25 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__25 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__26 {}
impl Clone for __or__26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__26 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__27 {}
impl Clone for __or__27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__27 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__28 {}
impl Clone for __or__28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__28 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__29 {}
impl Clone for __or__29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__30 {}
impl Clone for __or__30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__30 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__31 {}
impl Clone for __or__31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__32 {}
impl Clone for __or__32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__33 {}
impl Clone for __or__33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__34 {}
impl Clone for __or__34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__35 {}
impl Clone for __and__35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__36 {}
impl Clone for __and__36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__37 {}
impl Clone for __and__37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__38 {}
impl Clone for __and__38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__39 {}
impl Clone for __and__39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__40 {}
impl Clone for __and__40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__41 {}
impl Clone for __and__41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__42 {}
impl Clone for __and__42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__43 {}
impl Clone for __and__43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__44 {}
impl Clone for __not__44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__45 {}
impl Clone for __not__45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_46 {}
impl Clone for is_reference_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_47 {}
impl Clone for is_reference_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_48 {}
impl Clone for is_reference_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_49 {}
impl Clone for is_reference_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_50 {}
impl Clone for is_reference_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_51 {}
impl Clone for is_reference_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_52 {}
impl Clone for is_function_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_53 {}
impl Clone for is_function_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_54 {}
impl Clone for is_function_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_55 {}
impl Clone for is_function_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_56 {}
impl Clone for is_void_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_57 {}
impl Clone for is_void_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_58 {}
impl Clone for is_void_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_59 {}
impl Clone for is_void_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_60 {}
impl Clone for is_void_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_61 {}
impl Clone for remove_cv_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_62 {}
impl Clone for remove_cv_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_63 {}
impl Clone for remove_cv_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_64 {}
impl Clone for remove_cv_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_65 {}
impl Clone for remove_cv_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_66 {}
impl Clone for remove_cv_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_67 {}
impl Clone for is_const_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_68 {}
impl Clone for is_const_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_69 {}
impl Clone for is_const_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_70 {}
impl Clone for is_const_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_71 {}
impl Clone for is_const_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_72 {}
impl Clone for __is_array_unknown_bounds_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_73 {}
impl Clone for __is_array_unknown_bounds_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_74 {}
impl Clone for __is_array_unknown_bounds_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_75 {}
impl Clone for __is_array_unknown_bounds_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_76 {}
impl Clone for __is_integral_helper_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_77 {}
impl Clone for __is_integral_helper_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_78 {}
impl Clone for __is_integral_helper_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_79 {}
impl Clone for __is_integral_helper_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_80 {}
impl Clone for __is_integral_helper_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_81 {}
impl Clone for __is_integral_helper_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_82 {}
impl Clone for is_integral_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_83 {}
impl Clone for is_integral_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_84 {}
impl Clone for is_integral_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_85 {}
impl Clone for is_integral_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_86 {}
impl Clone for is_integral_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_87 {}
impl Clone for is_integral_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_88 {}
impl Clone for is_pointer_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_rvalue_reference_89 {}
impl Clone for is_rvalue_reference_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_rvalue_reference_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_object_pointer_90 {}
impl Clone for is_member_object_pointer_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_object_pointer_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_function_pointer_91 {}
impl Clone for is_member_function_pointer_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_function_pointer_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_92 {}
impl Clone for is_arithmetic_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_93 {}
impl Clone for is_scalar_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_94 {}
impl Clone for is_same_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_95 {}
impl Clone for is_volatile_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_96 {}
impl Clone for is_volatile_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_97 {}
impl Clone for is_volatile_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_98 {}
impl Clone for is_volatile_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_99 {}
impl Clone for is_volatile_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_empty_100 {}
impl Clone for is_empty_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_empty_101 {}
impl Clone for is_empty_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_102 {}
impl Clone for __is_destructible_safe_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_default_constructible_103 {}
impl Clone for is_default_constructible_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_default_constructible_104 {}
impl Clone for is_default_constructible_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_105 {}
impl Clone for is_move_constructible_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_106 {}
impl Clone for is_nothrow_default_constructible_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_107 {}
impl Clone for is_nothrow_default_constructible_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_108 {}
impl Clone for is_nothrow_copy_constructible_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_109 {}
impl Clone for is_nothrow_copy_constructible_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_110 {}
impl Clone for is_nothrow_move_constructible_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_assignable_111 {}
impl Clone for is_assignable_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_assignable_112 {}
impl Clone for is_assignable_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_assignable_113 {}
impl Clone for is_assignable_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_assignable_114 {}
impl Clone for is_assignable_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_115 {}
impl Clone for is_move_assignable_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_impl_116 {}
impl Clone for __is_implicitly_default_constructible_impl_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_impl_117 {}
impl Clone for __is_implicitly_default_constructible_impl_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_safe_118 {}
impl Clone for __is_implicitly_default_constructible_safe_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_safe_119 {}
impl Clone for __is_implicitly_default_constructible_safe_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_120 {}
impl Clone for __is_implicitly_default_constructible_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_121 {}
impl Clone for __is_implicitly_default_constructible_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_trivially_destructible_122 {}
impl Clone for is_trivially_destructible_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_convertible_123 {}
impl Clone for is_convertible_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_convertible_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_124 {}
impl Clone for remove_reference_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_125 {}
impl Clone for remove_reference_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_126 {}
impl Clone for remove_reference_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_127 {}
impl Clone for remove_reference_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_128 {}
impl Clone for remove_reference_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_129 {}
impl Clone for remove_reference_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_130 {}
impl Clone for remove_reference_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_131 {}
impl Clone for remove_reference_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_132 {}
impl Clone for remove_reference_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_133 {}
impl Clone for remove_reference_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct add_lvalue_reference_134 {}
impl Clone for add_lvalue_reference_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_lvalue_reference_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_135 {}
impl Clone for __cv_selector_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_136 {}
impl Clone for __cv_selector_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_137 {}
impl Clone for __cv_selector_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_138 {}
impl Clone for __cv_selector_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_139 {}
impl Clone for __match_cv_qualifiers_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_140 {}
impl Clone for __match_cv_qualifiers_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_141 {}
impl Clone for __match_cv_qualifiers_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_142 {}
impl Clone for __match_cv_qualifiers_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_142 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_143 {}
impl Clone for __match_cv_qualifiers_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_144 {}
impl Clone for __make_unsigned_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_144 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_145 {}
impl Clone for __make_unsigned_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_146 {}
impl Clone for __make_unsigned_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_147 {}
impl Clone for __make_unsigned_selector_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_148 {}
impl Clone for __make_unsigned_selector_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_149 {}
impl Clone for __make_unsigned_selector_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_150 {}
impl Clone for __make_unsigned_selector_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_151 {}
impl Clone for __make_unsigned_selector_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_152 {}
impl Clone for __make_unsigned_selector_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_156 {}
impl Clone for _List_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_157 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_158 {}
impl Clone for _List_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_159 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_160 {}
impl Clone for _List_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_161 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_162 {}
impl Clone for _List_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_163 {}
impl Clone for _List_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_164 {}
impl Clone for _List_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_165 {}
impl Clone for __select_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_166 {}
impl Clone for __select_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_167 {}
impl Clone for __select_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_168 {}
impl Clone for __select_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_169 {}
impl Clone for __select_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_170 {}
impl Clone for __make_signed_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_171 {}
impl Clone for __make_signed_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_172 {}
impl Clone for __make_signed_selector_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_173 {}
impl Clone for __make_signed_selector_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_174 {}
impl Clone for __make_signed_selector_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_175 {}
impl Clone for __make_signed_selector_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_176 {}
impl Clone for __make_signed_selector_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_pointer_177 {}
impl Clone for remove_pointer_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct common_type_178 {}
impl Clone for common_type_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __success_type_179 {}
impl Clone for __success_type_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __success_type_179 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __result_of_success_180 {}
impl Clone for __result_of_success_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_success_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __result_of_impl_181 {}
impl Clone for __result_of_impl_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_impl_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __invoke_result_182 {}
impl Clone for __invoke_result_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __invoke_result_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_invocable_impl_183 {}
impl Clone for __is_invocable_impl_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_impl_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_invocable_184 {}
impl Clone for __is_invocable_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
    pub fn copy_assign(&self, _a0: Ptr<std___nonesuchbase>) -> Ptr<std___nonesuchbase> {
        unimplemented!()
    }
}
impl std___nonesuchbase {
    pub fn move_assign(&self, _a0: Ptr<std___nonesuchbase>) -> Ptr<std___nonesuchbase> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_185 {}
impl Clone for __new_allocator_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_185 {
    pub fn copy_from(_a0: Ptr<__new_allocator_185>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_185 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_185>) -> Ptr<__new_allocator_185> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_186 {}
impl Clone for __new_allocator_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_186 {
    pub fn copy_from(_a0: Ptr<__new_allocator_186>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_186 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_186>) -> Ptr<__new_allocator_186> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_187 {}
impl Clone for __new_allocator_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_187 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_187 {
    pub fn copy_from(_a0: Ptr<__new_allocator_187>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_187 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_187>) -> Ptr<__new_allocator_187> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_188 {}
impl Clone for iterator_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_189 {}
impl Clone for iterator_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_189 {}
impl iterator_189 {}
impl iterator_189 {}
impl iterator_189 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_189>) -> Ptr<iterator_189> {
        unimplemented!()
    }
}
impl iterator_189 {
    pub fn move_assign(&self, _a0: Ptr<iterator_189>) -> Ptr<iterator_189> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_190 {}
impl Clone for iterator_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_190 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_190>) -> Ptr<iterator_190> {
        unimplemented!()
    }
}
impl iterator_190 {
    pub fn move_assign(&self, _a0: Ptr<iterator_190>) -> Ptr<iterator_190> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_191 {}
impl Clone for iterator_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_191 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_191>) -> Ptr<iterator_191> {
        unimplemented!()
    }
}
impl iterator_191 {
    pub fn move_assign(&self, _a0: Ptr<iterator_191>) -> Ptr<iterator_191> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_192 {}
impl Clone for iterator_traits_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_193 {}
impl Clone for iterator_traits_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_194 {}
impl Clone for iterator_traits_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_195 {}
impl Clone for iterator_traits_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_196 {}
impl Clone for __iterator_traits_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_197 {}
impl Clone for __iterator_traits_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_element_198 {}
impl Clone for tuple_element_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_element_199 {}
impl Clone for tuple_element_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Nth_type_200 {}
impl Clone for _Nth_type_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Nth_type_201 {}
impl Clone for _Nth_type_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
thread_local!(
    pub static piecewise_construct_202: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_203 {}
impl Clone for tuple_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_203 {}
impl tuple_203 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct tuple_204 {}
impl Clone for tuple_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_204 {
    pub fn copy_from(_a0: Ptr<tuple_204>) -> Self {
        unimplemented!()
    }
}
impl tuple_204 {
    pub fn move_from(_a0: Ptr<tuple_204>) -> Self {
        unimplemented!()
    }
}
impl tuple_204 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_205 {}
impl Clone for __are_same_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_206 = u32;
pub const anon_206___value: anon_206 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_207 {}
impl Clone for __are_same_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_208 = u32;
pub const anon_208___value: anon_208 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_209 {}
impl Clone for __are_same_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_210 = u32;
pub const anon_210___value: anon_210 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_211 {}
impl Clone for __are_same_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_212 = u32;
pub const anon_212___value: anon_212 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_213 {}
impl Clone for __are_same_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_214 = u32;
pub const anon_214___value: anon_214 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_215 {}
impl Clone for __are_same_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_216 = u32;
pub const anon_216___value: anon_216 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_217 {}
impl Clone for __is_integer_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_218 = u32;
pub const anon_218___value: anon_218 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_219 {}
impl Clone for __is_integer_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_220 = u32;
pub const anon_220___value: anon_220 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_221 {}
impl Clone for __is_integer_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_222 = u32;
pub const anon_222___value: anon_222 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_223 {}
impl Clone for __is_integer_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_224 = u32;
pub const anon_224___value: anon_224 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_225 {}
impl Clone for __is_integer_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_226 = u32;
pub const anon_226___value: anon_226 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_227 {}
impl Clone for __is_nonvolatile_trivially_copyable_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_228 = u32;
pub const anon_228___value: anon_228 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_229 {}
impl Clone for __memcpyable_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_230 {}
impl Clone for __memcpyable_integer_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_231 {}
impl Clone for __is_integer_nonstrict_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_232 = u32;
pub const anon_232___width: anon_232 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_233 {}
impl Clone for __is_integer_nonstrict_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_234 = u32;
pub const anon_234___width: anon_234 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_235 {}
impl Clone for __numeric_traits_integer_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_236: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_237: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_238: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_239 {}
impl Clone for __numeric_traits_integer_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_240: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_241: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_242: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_243 {}
impl Clone for __numeric_traits_floating_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_244 {}
impl Clone for __numeric_traits_floating_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_245 {}
impl Clone for __numeric_traits_floating_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_246 {}
impl Clone for __numeric_traits_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_247 {}
impl Clone for reverse_iterator_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_247 {
    pub fn copy_from(__x: Ptr<reverse_iterator_247>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_247 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_247>) -> Ptr<reverse_iterator_247> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_248 {}
impl Clone for reverse_iterator_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_248 {
    pub fn copy_from(__x: Ptr<reverse_iterator_248>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_248 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_248>) -> Ptr<reverse_iterator_248> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_249 {}
impl Clone for __normal_iterator_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_249 {
    pub fn new_1(__i: Ptr<Ptr<i32>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_249 {}
impl __normal_iterator_249 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_251 {}
impl Clone for __alloc_traits_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_252 {}
impl Clone for rebind_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __alloc_traits_251 {
    pub fn _S_select_on_copy(__a: Ptr<allocator_3>) -> allocator_3 {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
thread_local!(
    pub static allocator_arg_253: Value<std_allocator_arg_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___uses_alloc0 {
    #[offset(0)]
    #[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_empty_non_tuple_254 {}
impl Clone for __is_empty_non_tuple_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_empty_non_tuple_255 {}
impl Clone for __is_empty_non_tuple_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Head_base_256 {
    #[offset(0)]
    #[byte_size(1)]
    pub _M_head_impl: default_delete_257,
}
impl Clone for _Head_base_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_256 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_256 {
    pub fn copy_from(_a0: Ptr<_Head_base_256>) -> Self {
        unimplemented!()
    }
}
impl _Head_base_256 {
    pub fn move_from(_a0: Ptr<_Head_base_256>) -> Self {
        unimplemented!()
    }
}
impl _Head_base_256 {
    pub fn _M_head_12(__b: Ptr<_Head_base_256>) -> Ptr<default_delete_257> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct _Head_base_258 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_head_impl: Ptr<i32>,
}
impl Clone for _Head_base_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_258 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_258 {
    pub fn copy_from(_a0: Ptr<_Head_base_258>) -> Self {
        unimplemented!()
    }
}
impl _Head_base_258 {
    pub fn move_from(_a0: Ptr<_Head_base_258>) -> Self {
        unimplemented!()
    }
}
impl _Head_base_258 {
    pub fn _M_head_12(__b: Ptr<_Head_base_258>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl _Head_base_258 {
    pub fn _M_head_13(__b: Ptr<_Head_base_258>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct _Tuple_impl_259 {}
impl Clone for _Tuple_impl_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_259 {
    pub fn _M_head_1(__t: Ptr<_Tuple_impl_259>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl _Tuple_impl_259 {
    pub fn _M_head_2(__t: Ptr<_Tuple_impl_259>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl _Tuple_impl_259 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_259 {
    pub fn copy_from(_a0: Ptr<_Tuple_impl_259>) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_259 {
    pub fn move_from(_a0: Ptr<_Tuple_impl_259>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Tuple_impl_260 {}
impl Clone for _Tuple_impl_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_260 {
    pub fn _M_head_1(__t: Ptr<_Tuple_impl_260>) -> Ptr<default_delete_257> {
        unimplemented!()
    }
}
impl _Tuple_impl_260 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_260 {
    pub fn copy_from(_a0: Ptr<_Tuple_impl_260>) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_260 {
    pub fn move_from(__in: Ptr<_Tuple_impl_260>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _TupleConstraints_261 {}
impl Clone for _TupleConstraints_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _TupleConstraints_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _TupleConstraints_261 {
    pub fn __is_implicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
impl _TupleConstraints_261 {
    pub fn __is_explicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_262 {}
impl Clone for binary_function_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_262 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_263 {}
impl Clone for greater_263 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_263 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_264 {}
impl Clone for less_264 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_264 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_265 {}
impl Clone for greater_equal_265 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_265 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_266 {}
impl Clone for less_equal_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_267 {}
impl Clone for __hash_base_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_268 {}
impl Clone for __hash_base_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_269 {}
impl Clone for __hash_base_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_270 {}
impl Clone for __hash_base_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_271 {}
impl Clone for __hash_base_271 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_271 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_272 {}
impl Clone for __hash_base_272 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_272 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_273 {}
impl Clone for __hash_base_273 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_273 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_274 {}
impl Clone for __hash_base_274 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_274 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_275 {}
impl Clone for __hash_base_275 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_275 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_276 {}
impl Clone for __hash_base_276 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_276 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_277 {}
impl Clone for __hash_base_277 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_277 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_278 {}
impl Clone for __hash_base_278 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_278 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_279 {}
impl Clone for __hash_base_279 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_279 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_280 {}
impl Clone for __hash_base_280 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_280 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_281 {}
impl Clone for __hash_base_281 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_281 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_282 {}
impl Clone for __hash_base_282 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_282 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_283 {}
impl Clone for __hash_base_283 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_283 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_284 {}
impl Clone for __hash_base_284 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_284 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_285 {}
impl Clone for __hash_base_285 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_285 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_286 {}
impl Clone for __hash_base_286 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_286 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_287 {}
impl Clone for __hash_base_287 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_287 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_288 {}
impl Clone for hash_288 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_288 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
    pub fn hash(mut __ptr: AnyPtr, mut __clength: usize, __seed: Option<usize>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_289(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_290(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct default_delete_257 {}
impl Clone for default_delete_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for default_delete_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl default_delete_257 {}
impl default_delete_257 {}
impl default_delete_257 {
    pub fn copy_assign(&self, _a0: Ptr<default_delete_257>) -> Ptr<default_delete_257> {
        unimplemented!()
    }
}
impl default_delete_257 {
    pub fn move_assign(&self, _a0: Ptr<default_delete_257>) -> Ptr<default_delete_257> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __uniq_ptr_impl_291 {}
impl Clone for __uniq_ptr_impl_291 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_impl_291 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Ptr_292 {}
impl Clone for _Ptr_292 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_292 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_291 {
    pub fn new_1(mut __p: Ptr<i32>) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_291 {
    pub fn move_from(__u: Ptr<__uniq_ptr_impl_291>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __uniq_ptr_data_293 {}
impl Clone for __uniq_ptr_data_293 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_data_293 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_293 {
    pub fn move_from(_a0: Ptr<__uniq_ptr_data_293>) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_293 {
    pub fn move_assign(&self, _a0: Ptr<__uniq_ptr_data_293>) -> Ptr<__uniq_ptr_data_293> {
        unimplemented!()
    }
}
impl __uniq_ptr_data_293 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct unique_ptr_294 {}
impl Clone for unique_ptr_294 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for unique_ptr_294 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl unique_ptr_294 {
    pub fn move_from(_a0: Ptr<unique_ptr_294>) -> Self {
        unimplemented!()
    }
}
impl unique_ptr_294 {
    pub fn move_assign(&self, _a0: Ptr<unique_ptr_294>) -> Ptr<unique_ptr_294> {
        unimplemented!()
    }
}
impl unique_ptr_294 {
    pub fn new(mut __p: Ptr<i32>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _MakeUniq_295 {}
impl Clone for _MakeUniq_295 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _MakeUniq_295 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __mbstate_t {
    #[offset(0)]
    pub __count: i32,
    #[offset(4)]
    #[byte_size(4)]
    pub __value: anon_296,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct anon_296 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
}
impl Clone for anon_296 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_296 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(232)]
pub struct __locale_struct {
    #[offset(0)]
    #[byte_size(104)]
    pub __locales: Value<Box<[Ptr<__locale_data>]>>,
    #[offset(104)]
    #[byte_size(8)]
    pub __ctype_b: Ptr<u16>,
    #[offset(112)]
    #[byte_size(8)]
    pub __ctype_tolower: Ptr<i32>,
    #[offset(120)]
    #[byte_size(8)]
    pub __ctype_toupper: Ptr<i32>,
    #[offset(128)]
    #[byte_size(104)]
    pub __names: Value<Box<[Ptr<i8>]>>,
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __fsid_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __val: Value<Box<[i32]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timespec {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct sched_param {
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct cpu_set_t {
    #[offset(0)]
    #[byte_size(128)]
    pub __bits: Value<Box<[u64]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct timeval {
    #[offset(0)]
    pub tv_sec: i64,
    #[offset(8)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(208)]
pub struct timex {
    #[offset(0)]
    pub modes: u32,
    #[offset(8)]
    pub offset: i64,
    #[offset(16)]
    pub freq: i64,
    #[offset(24)]
    pub maxerror: i64,
    #[offset(32)]
    pub esterror: i64,
    #[offset(40)]
    pub status: i32,
    #[offset(48)]
    pub constant: i64,
    #[offset(56)]
    pub precision: i64,
    #[offset(64)]
    pub tolerance: i64,
    #[offset(72)]
    #[byte_size(16)]
    pub time: timeval,
    #[offset(88)]
    pub tick: i64,
    #[offset(96)]
    pub ppsfreq: i64,
    #[offset(104)]
    pub jitter: i64,
    #[offset(112)]
    pub shift: i32,
    #[offset(120)]
    pub stabil: i64,
    #[offset(128)]
    pub jitcnt: i64,
    #[offset(136)]
    pub calcnt: i64,
    #[offset(144)]
    pub errcnt: i64,
    #[offset(152)]
    pub stbcnt: i64,
    #[offset(160)]
    pub tai: i32,
    #[offset(164)]
    pub anon_297: i32,
    #[offset(168)]
    pub anon_298: i32,
    #[offset(172)]
    pub anon_299: i32,
    #[offset(176)]
    pub anon_300: i32,
    #[offset(180)]
    pub anon_301: i32,
    #[offset(184)]
    pub anon_302: i32,
    #[offset(188)]
    pub anon_303: i32,
    #[offset(192)]
    pub anon_304: i32,
    #[offset(196)]
    pub anon_305: i32,
    #[offset(200)]
    pub anon_306: i32,
    #[offset(204)]
    pub anon_307: i32,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct tm {
    #[offset(0)]
    pub tm_sec: i32,
    #[offset(4)]
    pub tm_min: i32,
    #[offset(8)]
    pub tm_hour: i32,
    #[offset(12)]
    pub tm_mday: i32,
    #[offset(16)]
    pub tm_mon: i32,
    #[offset(20)]
    pub tm_year: i32,
    #[offset(24)]
    pub tm_wday: i32,
    #[offset(28)]
    pub tm_yday: i32,
    #[offset(32)]
    pub tm_isdst: i32,
    #[offset(40)]
    pub tm_gmtoff: i64,
    #[offset(48)]
    #[byte_size(8)]
    pub tm_zone: Ptr<i8>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct itimerspec {
    #[offset(0)]
    #[byte_size(16)]
    pub it_interval: timespec,
    #[offset(16)]
    #[byte_size(16)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __atomic_wide_counter {
    #[offset(0)]
    pub __value64: u64,
    #[offset(0)]
    #[byte_size(8)]
    pub __value32: anon_308,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct anon_308 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_308 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_308 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct __pthread_internal_list {
    #[offset(0)]
    #[byte_size(8)]
    pub __prev: Ptr<__pthread_internal_list>,
    #[offset(8)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_list>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct __pthread_internal_slist {
    #[offset(0)]
    #[byte_size(8)]
    pub __next: Ptr<__pthread_internal_slist>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct __pthread_mutex_s {
    #[offset(0)]
    pub __lock: i32,
    #[offset(4)]
    pub __count: u32,
    #[offset(8)]
    pub __owner: i32,
    #[offset(12)]
    pub __nusers: u32,
    #[offset(16)]
    pub __kind: i32,
    #[offset(20)]
    pub __spins: i16,
    #[offset(22)]
    pub __elision: i16,
    #[offset(24)]
    #[byte_size(16)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct __pthread_rwlock_arch_t {
    #[offset(0)]
    pub __readers: u32,
    #[offset(4)]
    pub __writers: u32,
    #[offset(8)]
    pub __wrphase_futex: u32,
    #[offset(12)]
    pub __writers_futex: u32,
    #[offset(16)]
    pub __pad3: u32,
    #[offset(20)]
    pub __pad4: u32,
    #[offset(24)]
    pub __cur_writer: i32,
    #[offset(28)]
    pub __shared: i32,
    #[offset(32)]
    pub __rwelision: i8,
    #[offset(33)]
    #[byte_size(7)]
    pub __pad1: Value<Box<[u8]>>,
    #[offset(40)]
    pub __pad2: u64,
    #[offset(48)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct __pthread_cond_s {
    #[offset(0)]
    #[byte_size(8)]
    pub __wseq: __atomic_wide_counter,
    #[offset(8)]
    #[byte_size(8)]
    pub __g1_start: __atomic_wide_counter,
    #[offset(16)]
    #[byte_size(8)]
    pub __g_refs: Value<Box<[u32]>>,
    #[offset(24)]
    #[byte_size(8)]
    pub __g_size: Value<Box<[u32]>>,
    #[offset(32)]
    pub __g1_orig_size: u32,
    #[offset(36)]
    pub __wrefs: u32,
    #[offset(40)]
    #[byte_size(8)]
    pub __g_signals: Value<Box<[u32]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct __once_flag {
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_mutexattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_condattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_attr_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(40)]
pub struct pthread_mutex_t {
    #[offset(0)]
    #[byte_size(40)]
    pub __data: __pthread_mutex_s,
    #[offset(0)]
    #[byte_size(40)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(48)]
pub struct pthread_cond_t {
    #[offset(0)]
    #[byte_size(48)]
    pub __data: __pthread_cond_s,
    #[offset(0)]
    #[byte_size(48)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(56)]
pub struct pthread_rwlock_t {
    #[offset(0)]
    #[byte_size(56)]
    pub __data: __pthread_rwlock_arch_t,
    #[offset(0)]
    #[byte_size(56)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct pthread_rwlockattr_t {
    #[offset(0)]
    #[byte_size(8)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct pthread_barrier_t {
    #[offset(0)]
    #[byte_size(32)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(4)]
pub struct pthread_barrierattr_t {
    #[offset(0)]
    #[byte_size(4)]
    pub __size: Value<Box<[i8]>>,
    #[offset(0)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(128)]
pub struct __sigset_t {
    #[offset(0)]
    #[byte_size(128)]
    pub __val: Value<Box<[u64]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(200)]
pub struct __jmp_buf_tag {
    #[offset(0)]
    #[byte_size(64)]
    pub __jmpbuf: Value<Box<[i64]>>,
    #[offset(64)]
    pub __mask_was_saved: i32,
    #[offset(72)]
    #[byte_size(128)]
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
pub type anon_309 = u32;
pub const anon_309_PTHREAD_MUTEX_TIMED_NP: anon_309 = 0;
pub const anon_309_PTHREAD_MUTEX_RECURSIVE_NP: anon_309 = 1;
pub const anon_309_PTHREAD_MUTEX_ERRORCHECK_NP: anon_309 = 2;
pub const anon_309_PTHREAD_MUTEX_ADAPTIVE_NP: anon_309 = 3;
pub const anon_309_PTHREAD_MUTEX_NORMAL: anon_309 = 0;
pub const anon_309_PTHREAD_MUTEX_RECURSIVE: anon_309 = 1;
pub const anon_309_PTHREAD_MUTEX_ERRORCHECK: anon_309 = 2;
pub const anon_309_PTHREAD_MUTEX_DEFAULT: anon_309 = 0;
pub const anon_309_PTHREAD_MUTEX_FAST_NP: anon_309 = 0;
pub type anon_310 = u32;
pub const anon_310_PTHREAD_MUTEX_STALLED: anon_310 = 0;
pub const anon_310_PTHREAD_MUTEX_STALLED_NP: anon_310 = 0;
pub const anon_310_PTHREAD_MUTEX_ROBUST: anon_310 = 1;
pub const anon_310_PTHREAD_MUTEX_ROBUST_NP: anon_310 = 1;
pub type anon_311 = u32;
pub const anon_311_PTHREAD_RWLOCK_PREFER_READER_NP: anon_311 = 0;
pub const anon_311_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_311 = 1;
pub const anon_311_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_311 = 2;
pub const anon_311_PTHREAD_RWLOCK_DEFAULT_NP: anon_311 = 0;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct _pthread_cleanup_buffer {
    #[offset(0)]
    #[byte_size(8)]
    pub __routine: FnPtr<fn(AnyPtr)>,
    #[offset(8)]
    #[byte_size(8)]
    pub __arg: AnyPtr,
    #[offset(16)]
    pub __canceltype: i32,
    #[offset(24)]
    #[byte_size(8)]
    pub __prev: Ptr<_pthread_cleanup_buffer>,
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
pub type anon_312 = u32;
pub const anon_312_PTHREAD_CANCEL_DEFERRED: anon_312 = 0;
pub const anon_312_PTHREAD_CANCEL_ASYNCHRONOUS: anon_312 = 1;
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(72)]
pub struct __cancel_jmp_buf_tag {
    #[offset(0)]
    #[byte_size(64)]
    pub __cancel_jmp_buf: Value<Box<[i64]>>,
    #[offset(64)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(104)]
pub struct __pthread_unwind_buf_t {
    #[offset(0)]
    #[byte_size(72)]
    pub __cancel_jmp_buf: Value<Box<[__cancel_jmp_buf_tag]>>,
    #[offset(72)]
    #[byte_size(32)]
    pub __pad: Value<Box<[AnyPtr]>>,
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct __pthread_cleanup_frame {
    #[offset(0)]
    #[byte_size(8)]
    pub __cancel_routine: FnPtr<fn(AnyPtr)>,
    #[offset(8)]
    #[byte_size(8)]
    pub __cancel_arg: AnyPtr,
    #[offset(16)]
    pub __do_it: i32,
    #[offset(20)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
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
thread_local!(
    pub static __libc_single_threaded_313: Value<i8> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(32)]
pub struct max_align_t {
    #[offset(0)]
    pub __clang_max_align_nonce1: i64,
    #[offset(16)]
    #[byte_size(16)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
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
    pub fn copy_from(__other: Ptr<std___exception_ptr_exception_ptr>) -> Self {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptr {
    pub fn move_from(__o: Ptr<std___exception_ptr_exception_ptr>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
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
    pub fn copy_from(_a0: Ptr<std_nested_exception>) -> Self {
        unimplemented!()
    }
}
impl std_nested_exception {
    pub fn copy_assign(&self, _a0: Ptr<std_nested_exception>) -> Ptr<std_nested_exception> {
        unimplemented!()
    }
}
pub type __gnu_cxx__Lock_policy = u32;
pub const __gnu_cxx__Lock_policy__S_single: __gnu_cxx__Lock_policy = 0;
pub const __gnu_cxx__Lock_policy__S_mutex: __gnu_cxx__Lock_policy = 1;
pub const __gnu_cxx__Lock_policy__S_atomic: __gnu_cxx__Lock_policy = 2;
thread_local!(
    pub static __default_lock_policy_314: Value<__gnu_cxx__Lock_policy> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
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
    fn what(&self) -> Ptr<i8> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {}
impl __gnu_cxx___concurrence_lock_error {}
impl __gnu_cxx___concurrence_lock_error {
    pub fn copy_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_lock_error>,
    ) -> Ptr<__gnu_cxx___concurrence_lock_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {
    pub fn move_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_lock_error>,
    ) -> Ptr<__gnu_cxx___concurrence_lock_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_lock_error {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
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
    fn what(&self) -> Ptr<i8> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {}
impl __gnu_cxx___concurrence_unlock_error {}
impl __gnu_cxx___concurrence_unlock_error {
    pub fn copy_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_unlock_error>,
    ) -> Ptr<__gnu_cxx___concurrence_unlock_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {
    pub fn move_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_unlock_error>,
    ) -> Ptr<__gnu_cxx___concurrence_unlock_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_unlock_error {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
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
    fn what(&self) -> Ptr<i8> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {}
impl __gnu_cxx___concurrence_broadcast_error {}
impl __gnu_cxx___concurrence_broadcast_error {
    pub fn copy_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_broadcast_error>,
    ) -> Ptr<__gnu_cxx___concurrence_broadcast_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {
    pub fn move_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_broadcast_error>,
    ) -> Ptr<__gnu_cxx___concurrence_broadcast_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_broadcast_error {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
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
    fn what(&self) -> Ptr<i8> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {}
impl __gnu_cxx___concurrence_wait_error {}
impl __gnu_cxx___concurrence_wait_error {
    pub fn copy_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_wait_error>,
    ) -> Ptr<__gnu_cxx___concurrence_wait_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {
    pub fn move_assign(
        &self,
        _a0: Ptr<__gnu_cxx___concurrence_wait_error>,
    ) -> Ptr<__gnu_cxx___concurrence_wait_error> {
        unimplemented!()
    }
}
impl __gnu_cxx___concurrence_wait_error {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(40)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(40)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
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
    pub fn new(__name: Ptr<__gnu_cxx___mutex>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(48)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
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
    pub fn copy_assign(&self, _a0: Ptr<std_bad_weak_ptr>) -> Ptr<std_bad_weak_ptr> {
        unimplemented!()
    }
}
impl std_bad_weak_ptr {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(40)]
pub struct _Mutex_base_315 {}
impl Clone for _Mutex_base_315 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_315 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_316 = u32;
pub const anon_316__S_need_barriers: anon_316 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Mutex_base_317 {}
impl Clone for _Mutex_base_317 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_317 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Mutex_base_317 {}
impl _Mutex_base_317 {}
impl _Mutex_base_317 {
    pub fn copy_assign(&self, _a0: Ptr<_Mutex_base_317>) -> Ptr<_Mutex_base_317> {
        unimplemented!()
    }
}
impl _Mutex_base_317 {
    pub fn move_assign(&self, _a0: Ptr<_Mutex_base_317>) -> Ptr<_Mutex_base_317> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Mutex_base_318 {}
impl Clone for _Mutex_base_318 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_318 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_319 = u32;
pub const anon_319__S_need_barriers: anon_319 = 0;
impl _Mutex_base_318 {}
impl _Mutex_base_318 {}
impl _Mutex_base_318 {
    pub fn copy_assign(&self, _a0: Ptr<_Mutex_base_318>) -> Ptr<_Mutex_base_318> {
        unimplemented!()
    }
}
impl _Mutex_base_318 {
    pub fn move_assign(&self, _a0: Ptr<_Mutex_base_318>) -> Ptr<_Mutex_base_318> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Sp_counted_ptr_320 {}
impl Clone for _Sp_counted_ptr_320 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_320 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct _Sp_counted_ptr_321 {}
impl Clone for _Sp_counted_ptr_321 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_321 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Sp_counted_ptr_322 {}
impl Clone for _Sp_counted_ptr_322 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_322 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct _Sp_alloc_shared_tag_323 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_a: Ptr<allocator_0>,
}
impl Clone for _Sp_alloc_shared_tag_323 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_alloc_shared_tag_323 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Sp_owner_less_324 {}
impl Clone for _Sp_owner_less_324 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_owner_less_324 {
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
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(1)]
pub struct std___atomic_flag_base {
    #[offset(0)]
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
    pub fn copy_assign(&self, _a0: Ptr<std___atomic_flag_base>) -> Ptr<std___atomic_flag_base> {
        unimplemented!()
    }
}
impl std___atomic_flag_base {
    pub fn move_assign(&self, _a0: Ptr<std___atomic_flag_base>) -> Ptr<std___atomic_flag_base> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(2)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_325 {}
impl Clone for initializer_list_325 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_325 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_326 {}
impl Clone for initializer_list_326 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_326 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_326 {}
impl initializer_list_326 {}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_327 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_327 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_327 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_int__std_allocator_int____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<i32>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<i32>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<i32>,
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
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_int__std_allocator_int____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
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
    pub fn new_1(__a: Ptr<allocator_3>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_int__std_allocator_int____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_327 {
    pub fn new_1(__a: Ptr<allocator_3>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_327 {
    pub fn new_2(mut __n: usize, __a: Ptr<allocator_3>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_327 {
    pub fn move_from(_a0: Ptr<_Vector_base_327>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_328 {}
impl Clone for vector_328 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_328 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_328 {
    pub fn copy_from(__x: Ptr<vector_328>) -> Self {
        unimplemented!()
    }
}
impl vector_328 {
    pub fn move_from(_a0: Ptr<vector_328>) -> Self {
        unimplemented!()
    }
}
impl vector_328 {
    pub fn new_1(__l: initializer_list_326, __a: Option<Ptr<allocator_3>>) -> Self {
        unimplemented!()
    }
}
pub type anon_329 = u32;
pub const anon_329__S_word_bit: anon_329 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
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
    pub fn copy_from(_a0: Ptr<std__Bit_reference>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct std__Bit_iterator_base {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_p: Ptr<u64>,
    #[offset(8)]
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
    pub fn new(mut __x: Ptr<u64>, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {}
impl std__Bit_iterator_base {
    pub fn copy_assign(&self, _a0: Ptr<std__Bit_iterator_base>) -> Ptr<std__Bit_iterator_base> {
        unimplemented!()
    }
}
impl std__Bit_iterator_base {
    pub fn move_assign(&self, _a0: Ptr<std__Bit_iterator_base>) -> Ptr<std__Bit_iterator_base> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
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
    pub fn new_1(mut __x: Ptr<u64>, mut __y: u32) -> Self {
        unimplemented!()
    }
}
impl std__Bit_iterator {}
impl std__Bit_iterator {}
impl std__Bit_iterator {
    pub fn copy_assign(&self, _a0: Ptr<std__Bit_iterator>) -> Ptr<std__Bit_iterator> {
        unimplemented!()
    }
}
impl std__Bit_iterator {
    pub fn move_assign(&self, _a0: Ptr<std__Bit_iterator>) -> Ptr<std__Bit_iterator> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
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
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {}
impl std__Bit_const_iterator {
    pub fn copy_assign(&self, _a0: Ptr<std__Bit_const_iterator>) -> Ptr<std__Bit_const_iterator> {
        unimplemented!()
    }
}
impl std__Bit_const_iterator {
    pub fn move_assign(&self, _a0: Ptr<std__Bit_const_iterator>) -> Ptr<std__Bit_const_iterator> {
        unimplemented!()
    }
}
pub type std_byte = u8;
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let v: Value<vector_328> = Rc::new(RefCell::new(vector_328::new_1(
        { __cpp2rust_unimplemented::<initializer_list_326>() },
        None,
    )));
    let f: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let v: Value<vector_328> =
                Rc::new(RefCell::new(vector_328::copy_from({ v.as_pointer() })));
        },
        || -> i32 {
            return (({ vector_328Impl::size(&v.as_pointer()) }) as i32);
        }
    )));
    let g: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*f.borrow()).copy_from()));
    let h: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*f.borrow()).move_from()));
    (if (({ (*g.borrow()).call() }) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_331(
                Ptr::<i8>::from_string_literal(b"g() == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*h.borrow()).call() }) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_331(
                Ptr::<i8>::from_string_literal(b"h() == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*f.borrow()).call() }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_331(
                Ptr::<i8>::from_string_literal(b"f() == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let p: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let u: Value<unique_ptr_294> = Rc::new(RefCell::new(
                ({
                    let ___args: Value<i32> = Rc::new(RefCell::new(5));
                    make_unique_333(___args.as_pointer())
                }),
            ));
        },
        || -> i32 {
            return if !({ unique_ptr_294Impl::get(&u.as_pointer()) }).is_null() {
                (({ unique_ptr_294Impl::operator_deref(&u.as_pointer()) }).read())
            } else {
                0
            };
        }
    )));
    let q: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*p.borrow()).move_from()));
    (if (({ (*q.borrow()).call() }) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_331(
                Ptr::<i8>::from_string_literal(b"q() == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*p.borrow()).call() }) == 0) {
        &(0);
    } else {
        ({
            __assert_fail_331(
                Ptr::<i8>::from_string_literal(b"p() == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let n: Value<i32> = Rc::new(RefCell::new(0));
    let inner: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let n: Value<i32> = Rc::new(RefCell::new((*n.borrow())));
        },
        || -> i32 {
            return (*n.borrow_mut()).prefix_inc();
        }
    )));
    let outer: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new(lambda!(
        {
            let inner: Value<FnPtr<fn() -> i32>> =
                Rc::new(RefCell::new((*inner.borrow()).copy_from()));
        },
        || -> i32 {
            return ({ (*inner.borrow()).call() }).clone();
        }
    )));
    (if (({ (*outer.borrow()).call() }) == 1) {
        &(0);
    } else {
        ({
            __assert_fail_331(
                Ptr::<i8>::from_string_literal(b"outer() == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let outer2: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*outer.borrow()).copy_from()));
    (if (({ (*outer2.borrow()).call() }) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_331(
                Ptr::<i8>::from_string_literal(b"outer2() == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ (*outer.borrow()).call() }) == 2) {
        &(0);
    } else {
        ({
            __assert_fail_331(
                Ptr::<i8>::from_string_literal(b"outer() == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let outer3: Value<FnPtr<fn() -> i32>> = Rc::new(RefCell::new((*outer.borrow()).move_from()));
    (if (({ (*outer3.borrow()).call() }) == 3) {
        &(0);
    } else {
        ({
            __assert_fail_331(
                Ptr::<i8>::from_string_literal(b"outer3() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    return 0;
}
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std___uses_alloc0__Sink;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct __locale_data;
#[track_caller]
pub fn __cpp2rust_unimplemented<T>() -> T {
    unimplemented!()
}
pub fn __assert_fail_331(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub fn make_unique_333(__args: Ptr<i32>) -> unique_ptr_294 {
    unimplemented!()
}
pub trait _Vector_base_327Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_3> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_3> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_327Impl for Ptr<_Vector_base_327> {}
pub trait __gnu_cxx___mutexImpl {
    fn lock(&self) {
        unimplemented!()
    }
    fn unlock(&self) {
        unimplemented!()
    }
    fn gthread_mutex(&self) -> Ptr<pthread_mutex_t> {
        unimplemented!()
    }
}
impl __gnu_cxx___mutexImpl for Ptr<__gnu_cxx___mutex> {}
pub trait __gnu_cxx___recursive_mutexImpl {
    fn gthread_recursive_mutex(&self) -> Ptr<pthread_mutex_t> {
        unimplemented!()
    }
}
impl __gnu_cxx___recursive_mutexImpl for Ptr<__gnu_cxx___recursive_mutex> {}
pub trait __new_allocator_187Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<i32> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
}
impl __new_allocator_187Impl for Ptr<__new_allocator_187> {}
pub trait __normal_iterator_249Impl {
    fn base(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl __normal_iterator_249Impl for Ptr<__normal_iterator_249> {}
pub trait __uniq_ptr_impl_291Impl {
    fn move_assign(&self, __u: Ptr<__uniq_ptr_impl_291>) -> Ptr<__uniq_ptr_impl_291> {
        unimplemented!()
    }
    fn _M_ptr_6(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn _M_ptr_7(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deleter_8(&self) -> Ptr<default_delete_257> {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_291Impl for Ptr<__uniq_ptr_impl_291> {}
pub trait default_delete_257Impl {
    fn operator_call(&self, mut __ptr: Ptr<i32>) {
        unimplemented!()
    }
}
impl default_delete_257Impl for Ptr<default_delete_257> {}
pub trait initializer_list_325Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_325Impl for Ptr<initializer_list_325> {}
pub trait initializer_list_326Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn end(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl initializer_list_326Impl for Ptr<initializer_list_326> {}
pub trait integral_constant_6Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_6Impl for Ptr<integral_constant_6> {}
pub trait std__Bit_const_iteratorImpl {
    fn operator_deref(&self) -> bool {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut __i: i64) -> Ptr<std__Bit_const_iterator> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut __i: i64) -> Ptr<std__Bit_const_iterator> {
        unimplemented!()
    }
}
impl std__Bit_const_iteratorImpl for Ptr<std__Bit_const_iterator> {}
pub trait std__Bit_iteratorImpl {
    fn operator_deref(&self) -> std__Bit_reference {
        unimplemented!()
    }
    fn operator_add_assign(&self, mut __i: i64) -> Ptr<std__Bit_iterator> {
        unimplemented!()
    }
    fn operator_sub_assign(&self, mut __i: i64) -> Ptr<std__Bit_iterator> {
        unimplemented!()
    }
}
impl std__Bit_iteratorImpl for Ptr<std__Bit_iterator> {}
pub trait std__Bit_iterator_baseImpl {
    fn _M_assume_normalized(&self) {
        unimplemented!()
    }
    fn _M_bump_up(&self) {
        unimplemented!()
    }
    fn _M_bump_down(&self) {
        unimplemented!()
    }
    fn _M_incr(&self, mut __i: i64) {
        unimplemented!()
    }
}
impl std__Bit_iterator_baseImpl for Ptr<std__Bit_iterator_base> {}
pub trait std__Bit_referenceImpl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
    fn operator_assign_5(&self, mut __x: bool) -> Ptr<std__Bit_reference> {
        unimplemented!()
    }
    fn copy_assign(&self, __x: Ptr<std__Bit_reference>) -> Ptr<std__Bit_reference> {
        unimplemented!()
    }
}
impl std__Bit_referenceImpl for Ptr<std__Bit_reference> {}
pub trait std___exception_ptr_exception_ptrImpl {
    fn copy_assign(
        &self,
        __other: Ptr<std___exception_ptr_exception_ptr>,
    ) -> Ptr<std___exception_ptr_exception_ptr> {
        unimplemented!()
    }
    fn move_assign(
        &self,
        __o: Ptr<std___exception_ptr_exception_ptr>,
    ) -> Ptr<std___exception_ptr_exception_ptr> {
        unimplemented!()
    }
    fn swap(&self, __other: Ptr<std___exception_ptr_exception_ptr>) {
        unimplemented!()
    }
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl std___exception_ptr_exception_ptrImpl for Ptr<std___exception_ptr_exception_ptr> {}
pub trait std_type_infoImpl {
    fn name(&self) -> Ptr<i8> {
        unimplemented!()
    }
    fn operator_eq(&self, __arg: Ptr<std_type_info>) -> bool {
        unimplemented!()
    }
}
impl std_type_infoImpl for Ptr<std_type_info> {}
pub trait tuple_204Impl {
    fn copy_assign(&self, __in: Ptr<tuple_204>) -> Ptr<tuple_204> {
        unimplemented!()
    }
    fn move_assign(&self, __in: Ptr<tuple_204>) -> Ptr<tuple_204> {
        unimplemented!()
    }
}
impl tuple_204Impl for Ptr<tuple_204> {}
pub trait unique_ptr_294Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn get(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn get_deleter_17(&self) -> Ptr<default_delete_257> {
        unimplemented!()
    }
}
impl unique_ptr_294Impl for Ptr<unique_ptr_294> {}
pub trait vector_328Impl {
    fn copy_assign(&self, __x: Ptr<vector_328>) -> Ptr<vector_328> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_328>) -> Ptr<vector_328> {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_249 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_249 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
}
impl vector_328Impl for Ptr<vector_328> {}
pub fn __cpp2rust_init_globals() {}
