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
    pub static nothrow_3: Value<std_nothrow_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_4 {}
impl Clone for integral_constant_4 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_4 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__true__value_5: Value<bool> = unimplemented!();
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
    pub static std_integral_constant_bool__false__value_7: Value<bool> = unimplemented!();
);
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_9 {}
impl Clone for integral_constant_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_10 {}
impl Clone for enable_if_10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_10 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_11 {}
impl Clone for enable_if_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_11 {
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
pub struct __conditional_14 {}
impl Clone for __conditional_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_15 {}
impl Clone for __conditional_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_15 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_16 {}
impl Clone for __type_identity_16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_16 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_17 {}
impl Clone for __type_identity_17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_17 {
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
pub struct __or__20 {}
impl Clone for __or__20 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__20 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__21 {}
impl Clone for __or__21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__21 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__22 {}
impl Clone for __or__22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__22 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__23 {}
impl Clone for __or__23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__23 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__24 {}
impl Clone for __and__24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__24 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__25 {}
impl Clone for __and__25 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__25 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__26 {}
impl Clone for __and__26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__26 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__27 {}
impl Clone for __and__27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__27 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__28 {}
impl Clone for __and__28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__28 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__29 {}
impl Clone for __and__29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__30 {}
impl Clone for __and__30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__30 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__31 {}
impl Clone for __and__31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__32 {}
impl Clone for __not__32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__33 {}
impl Clone for __not__33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_34 {}
impl Clone for is_reference_34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_35 {}
impl Clone for is_reference_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_36 {}
impl Clone for is_reference_36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_37 {}
impl Clone for is_reference_37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_38 {}
impl Clone for is_function_38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_39 {}
impl Clone for is_function_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_40 {}
impl Clone for is_void_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_41 {}
impl Clone for is_void_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_42 {}
impl Clone for is_void_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_43 {}
impl Clone for is_void_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_44 {}
impl Clone for remove_cv_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_45 {}
impl Clone for remove_cv_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_46 {}
impl Clone for remove_cv_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_47 {}
impl Clone for remove_cv_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_48 {}
impl Clone for remove_cv_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_49 {}
impl Clone for is_const_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_50 {}
impl Clone for is_const_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_51 {}
impl Clone for is_const_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_52 {}
impl Clone for is_const_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_53 {}
impl Clone for is_const_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_54 {}
impl Clone for __is_array_unknown_bounds_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_55 {}
impl Clone for __is_array_unknown_bounds_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_56 {}
impl Clone for __is_integral_helper_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_57 {}
impl Clone for __is_integral_helper_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_58 {}
impl Clone for __is_integral_helper_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_59 {}
impl Clone for __is_integral_helper_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_60 {}
impl Clone for __is_integral_helper_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_61 {}
impl Clone for is_integral_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_62 {}
impl Clone for is_integral_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_63 {}
impl Clone for is_integral_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_64 {}
impl Clone for is_integral_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_65 {}
impl Clone for is_integral_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_66 {}
impl Clone for is_pointer_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_rvalue_reference_67 {}
impl Clone for is_rvalue_reference_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_rvalue_reference_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_object_pointer_68 {}
impl Clone for is_member_object_pointer_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_object_pointer_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_function_pointer_69 {}
impl Clone for is_member_function_pointer_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_function_pointer_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_70 {}
impl Clone for is_volatile_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_71 {}
impl Clone for is_volatile_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_72 {}
impl Clone for is_volatile_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_73 {}
impl Clone for is_volatile_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_74 {}
impl Clone for is_volatile_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_empty_75 {}
impl Clone for is_empty_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_empty_76 {}
impl Clone for is_empty_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_empty_76 {
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
pub struct is_default_constructible_77 {}
impl Clone for is_default_constructible_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_default_constructible_78 {}
impl Clone for is_default_constructible_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_default_constructible_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_79 {}
impl Clone for is_move_constructible_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_80 {}
impl Clone for is_nothrow_default_constructible_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_81 {}
impl Clone for is_nothrow_default_constructible_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_assignable_82 {}
impl Clone for is_assignable_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_assignable_83 {}
impl Clone for is_assignable_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_assignable_84 {}
impl Clone for is_assignable_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_assignable_85 {}
impl Clone for is_assignable_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_assignable_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_assignable_86 {}
impl Clone for is_move_assignable_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_assignable_86 {
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
pub struct __is_implicitly_default_constructible_impl_87 {}
impl Clone for __is_implicitly_default_constructible_impl_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_impl_88 {}
impl Clone for __is_implicitly_default_constructible_impl_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_impl_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_safe_89 {}
impl Clone for __is_implicitly_default_constructible_safe_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_safe_90 {}
impl Clone for __is_implicitly_default_constructible_safe_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_safe_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_91 {}
impl Clone for __is_implicitly_default_constructible_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_implicitly_default_constructible_92 {}
impl Clone for __is_implicitly_default_constructible_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_implicitly_default_constructible_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_93 {}
impl Clone for remove_reference_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_94 {}
impl Clone for remove_reference_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_95 {}
impl Clone for remove_reference_95 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_95 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_96 {}
impl Clone for remove_reference_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_97 {}
impl Clone for remove_reference_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct add_lvalue_reference_98 {}
impl Clone for add_lvalue_reference_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for add_lvalue_reference_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_99 {}
impl Clone for __cv_selector_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_100 {}
impl Clone for __cv_selector_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_101 {}
impl Clone for __cv_selector_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_102 {}
impl Clone for __cv_selector_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_103 {}
impl Clone for __match_cv_qualifiers_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_104 {}
impl Clone for __match_cv_qualifiers_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_105 {}
impl Clone for __match_cv_qualifiers_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_106 {}
impl Clone for __match_cv_qualifiers_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_107 {}
impl Clone for __match_cv_qualifiers_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_108 {}
impl Clone for __make_unsigned_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_109 {}
impl Clone for __make_unsigned_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_110 {}
impl Clone for __make_unsigned_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_111 {}
impl Clone for __make_unsigned_selector_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_112 {}
impl Clone for __make_unsigned_selector_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_113 {}
impl Clone for __make_unsigned_selector_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_114 {}
impl Clone for __make_unsigned_selector_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_115 {}
impl Clone for __make_unsigned_selector_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_116 {}
impl Clone for __make_unsigned_selector_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_116 {
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
pub struct _List_120 {}
impl Clone for _List_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_121 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_122 {}
impl Clone for _List_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_123 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_124 {}
impl Clone for _List_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_125 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_126 {}
impl Clone for _List_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_127 {}
impl Clone for _List_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_128 {}
impl Clone for _List_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_129 {}
impl Clone for __select_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_130 {}
impl Clone for __select_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_131 {}
impl Clone for __select_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_132 {}
impl Clone for __select_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_133 {}
impl Clone for __select_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_134 {}
impl Clone for __make_signed_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_135 {}
impl Clone for __make_signed_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_136 {}
impl Clone for __make_signed_selector_136 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_136 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_137 {}
impl Clone for __make_signed_selector_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_138 {}
impl Clone for __make_signed_selector_138 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_138 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_139 {}
impl Clone for __make_signed_selector_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_140 {}
impl Clone for __make_signed_selector_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_140 {
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
pub struct common_type_141 {}
impl Clone for common_type_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __success_type_142 {}
impl Clone for __success_type_142 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __success_type_142 {
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
pub struct __result_of_success_143 {}
impl Clone for __result_of_success_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_success_143 {
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
pub struct __result_of_impl_144 {}
impl Clone for __result_of_impl_144 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __result_of_impl_144 {
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
pub struct __invoke_result_145 {}
impl Clone for __invoke_result_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __invoke_result_145 {
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
pub struct __is_invocable_impl_146 {}
impl Clone for __is_invocable_impl_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_impl_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_invocable_147 {}
impl Clone for __is_invocable_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_invocable_147 {
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
pub struct __new_allocator_148 {}
impl Clone for __new_allocator_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_148 {
    pub fn copy_from(_a0: Ptr<__new_allocator_148>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_148 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_148>) -> Ptr<__new_allocator_148> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_149 {}
impl Clone for __new_allocator_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_149 {
    pub fn copy_from(_a0: Ptr<__new_allocator_149>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_149 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_149>) -> Ptr<__new_allocator_149> {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_150 {}
impl Clone for iterator_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_element_151 {}
impl Clone for tuple_element_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_element_152 {}
impl Clone for tuple_element_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_element_152 {
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
pub struct _Nth_type_153 {}
impl Clone for _Nth_type_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Nth_type_154 {}
impl Clone for _Nth_type_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Nth_type_154 {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_155 {}
impl Clone for tuple_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct tuple_156 {}
impl Clone for tuple_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_156 {
    pub fn copy_from(_a0: Ptr<tuple_156>) -> Self {
        unimplemented!()
    }
}
impl tuple_156 {
    pub fn move_from(_a0: Ptr<tuple_156>) -> Self {
        unimplemented!()
    }
}
impl tuple_156 {
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
pub struct __are_same_157 {}
impl Clone for __are_same_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_158 = u32;
pub const anon_158___value: anon_158 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_159 {}
impl Clone for __are_same_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_160 = u32;
pub const anon_160___value: anon_160 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_161 {}
impl Clone for __are_same_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_162 = u32;
pub const anon_162___value: anon_162 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_163 {}
impl Clone for __are_same_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_164 = u32;
pub const anon_164___value: anon_164 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_165 {}
impl Clone for __are_same_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_166 = u32;
pub const anon_166___value: anon_166 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_167 {}
impl Clone for __are_same_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_168 = u32;
pub const anon_168___value: anon_168 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_169 {}
impl Clone for __is_integer_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_170 = u32;
pub const anon_170___value: anon_170 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_171 {}
impl Clone for __is_integer_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_172 = u32;
pub const anon_172___value: anon_172 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_173 {}
impl Clone for __is_integer_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_174 = u32;
pub const anon_174___value: anon_174 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_175 {}
impl Clone for __is_integer_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_176 = u32;
pub const anon_176___value: anon_176 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_177 {}
impl Clone for __memcpyable_integer_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_178 {}
impl Clone for __is_integer_nonstrict_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_179 = u32;
pub const anon_179___width: anon_179 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_180 {}
impl Clone for __numeric_traits_integer_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_181: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_182: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_183: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_184 {}
impl Clone for __numeric_traits_floating_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_185 {}
impl Clone for __numeric_traits_floating_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_186 {}
impl Clone for __numeric_traits_floating_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_187 {}
impl Clone for __numeric_traits_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
    pub static allocator_arg_189: Value<std_allocator_arg_t> = unimplemented!();
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
pub struct __is_empty_non_tuple_190 {}
impl Clone for __is_empty_non_tuple_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_empty_non_tuple_191 {}
impl Clone for __is_empty_non_tuple_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_empty_non_tuple_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Head_base_192 {
    #[offset(0)]
    #[byte_size(1)]
    pub _M_head_impl: default_delete_193,
}
impl Clone for _Head_base_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_192 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_192 {
    pub fn copy_from(_a0: Ptr<_Head_base_192>) -> Self {
        unimplemented!()
    }
}
impl _Head_base_192 {
    pub fn move_from(_a0: Ptr<_Head_base_192>) -> Self {
        unimplemented!()
    }
}
impl _Head_base_192 {
    pub fn _M_head_12(__b: Ptr<_Head_base_192>) -> Ptr<default_delete_193> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(8)]
pub struct _Head_base_194 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_head_impl: Ptr<i32>,
}
impl Clone for _Head_base_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Head_base_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Head_base_194 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Head_base_194 {
    pub fn copy_from(_a0: Ptr<_Head_base_194>) -> Self {
        unimplemented!()
    }
}
impl _Head_base_194 {
    pub fn move_from(_a0: Ptr<_Head_base_194>) -> Self {
        unimplemented!()
    }
}
impl _Head_base_194 {
    pub fn _M_head_12(__b: Ptr<_Head_base_194>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl _Head_base_194 {
    pub fn _M_head_13(__b: Ptr<_Head_base_194>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct _Tuple_impl_195 {}
impl Clone for _Tuple_impl_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_195 {
    pub fn _M_head_1(__t: Ptr<_Tuple_impl_195>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl _Tuple_impl_195 {
    pub fn _M_head_2(__t: Ptr<_Tuple_impl_195>) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
}
impl _Tuple_impl_195 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_195 {
    pub fn copy_from(_a0: Ptr<_Tuple_impl_195>) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_195 {
    pub fn move_from(_a0: Ptr<_Tuple_impl_195>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Tuple_impl_196 {}
impl Clone for _Tuple_impl_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Tuple_impl_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_196 {
    pub fn _M_head_1(__t: Ptr<_Tuple_impl_196>) -> Ptr<default_delete_193> {
        unimplemented!()
    }
}
impl _Tuple_impl_196 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_196 {
    pub fn copy_from(_a0: Ptr<_Tuple_impl_196>) -> Self {
        unimplemented!()
    }
}
impl _Tuple_impl_196 {
    pub fn move_from(__in: Ptr<_Tuple_impl_196>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _TupleConstraints_197 {}
impl Clone for _TupleConstraints_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _TupleConstraints_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _TupleConstraints_197 {
    pub fn __is_implicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
impl _TupleConstraints_197 {
    pub fn __is_explicitly_default_constructible() -> bool {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_198 {}
impl Clone for binary_function_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_199 {}
impl Clone for greater_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_200 {}
impl Clone for less_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_201 {}
impl Clone for greater_equal_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_202 {}
impl Clone for less_equal_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_203 {}
impl Clone for __hash_base_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_204 {}
impl Clone for __hash_base_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_205 {}
impl Clone for __hash_base_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_206 {}
impl Clone for __hash_base_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_207 {}
impl Clone for __hash_base_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_208 {}
impl Clone for __hash_base_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_209 {}
impl Clone for __hash_base_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_210 {}
impl Clone for __hash_base_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_211 {}
impl Clone for __hash_base_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_212 {}
impl Clone for __hash_base_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_213 {}
impl Clone for __hash_base_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_214 {}
impl Clone for __hash_base_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_215 {}
impl Clone for __hash_base_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_216 {}
impl Clone for __hash_base_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_217 {}
impl Clone for __hash_base_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_218 {}
impl Clone for __hash_base_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_219 {}
impl Clone for __hash_base_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_220 {}
impl Clone for __hash_base_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_221 {}
impl Clone for __hash_base_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_222 {}
impl Clone for __hash_base_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_223 {}
impl Clone for __hash_base_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_224 {}
impl Clone for hash_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_224 {
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
    pub fn hash_225(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_226(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct default_delete_193 {}
impl Clone for default_delete_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for default_delete_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl default_delete_193 {}
impl default_delete_193 {}
impl default_delete_193 {
    pub fn copy_assign(&self, _a0: Ptr<default_delete_193>) -> Ptr<default_delete_193> {
        unimplemented!()
    }
}
impl default_delete_193 {
    pub fn move_assign(&self, _a0: Ptr<default_delete_193>) -> Ptr<default_delete_193> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __uniq_ptr_impl_227 {}
impl Clone for __uniq_ptr_impl_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_impl_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Ptr_228 {}
impl Clone for _Ptr_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Ptr_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_227 {
    pub fn new_1(mut __p: Ptr<i32>) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_227 {
    pub fn move_from(__u: Ptr<__uniq_ptr_impl_227>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __uniq_ptr_data_229 {}
impl Clone for __uniq_ptr_data_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __uniq_ptr_data_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_229 {
    pub fn move_from(_a0: Ptr<__uniq_ptr_data_229>) -> Self {
        unimplemented!()
    }
}
impl __uniq_ptr_data_229 {
    pub fn move_assign(&self, _a0: Ptr<__uniq_ptr_data_229>) -> Ptr<__uniq_ptr_data_229> {
        unimplemented!()
    }
}
impl __uniq_ptr_data_229 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct unique_ptr_230 {}
impl Clone for unique_ptr_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for unique_ptr_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl unique_ptr_230 {
    pub fn move_from(_a0: Ptr<unique_ptr_230>) -> Self {
        unimplemented!()
    }
}
impl unique_ptr_230 {
    pub fn move_assign(&self, _a0: Ptr<unique_ptr_230>) -> Ptr<unique_ptr_230> {
        unimplemented!()
    }
}
impl unique_ptr_230 {
    pub fn new(mut __p: Ptr<i32>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _MakeUniq_231 {}
impl Clone for _MakeUniq_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _MakeUniq_231 {
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
    pub __value: anon_232,
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
pub struct anon_232 {
    #[offset(0)]
    pub __wch: u32,
    #[offset(0)]
    #[byte_size(4)]
    pub __wchb: Value<Box<[i8]>>,
}
impl Clone for anon_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_232 {
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
    pub anon_233: i32,
    #[offset(168)]
    pub anon_234: i32,
    #[offset(172)]
    pub anon_235: i32,
    #[offset(176)]
    pub anon_236: i32,
    #[offset(180)]
    pub anon_237: i32,
    #[offset(184)]
    pub anon_238: i32,
    #[offset(188)]
    pub anon_239: i32,
    #[offset(192)]
    pub anon_240: i32,
    #[offset(196)]
    pub anon_241: i32,
    #[offset(200)]
    pub anon_242: i32,
    #[offset(204)]
    pub anon_243: i32,
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
    pub __value32: anon_244,
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
pub struct anon_244 {
    #[offset(0)]
    pub __low: u32,
    #[offset(4)]
    pub __high: u32,
}
impl Clone for anon_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for anon_244 {
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
pub type anon_245 = u32;
pub const anon_245_PTHREAD_MUTEX_TIMED_NP: anon_245 = 0;
pub const anon_245_PTHREAD_MUTEX_RECURSIVE_NP: anon_245 = 1;
pub const anon_245_PTHREAD_MUTEX_ERRORCHECK_NP: anon_245 = 2;
pub const anon_245_PTHREAD_MUTEX_ADAPTIVE_NP: anon_245 = 3;
pub const anon_245_PTHREAD_MUTEX_NORMAL: anon_245 = 0;
pub const anon_245_PTHREAD_MUTEX_RECURSIVE: anon_245 = 1;
pub const anon_245_PTHREAD_MUTEX_ERRORCHECK: anon_245 = 2;
pub const anon_245_PTHREAD_MUTEX_DEFAULT: anon_245 = 0;
pub const anon_245_PTHREAD_MUTEX_FAST_NP: anon_245 = 0;
pub type anon_246 = u32;
pub const anon_246_PTHREAD_MUTEX_STALLED: anon_246 = 0;
pub const anon_246_PTHREAD_MUTEX_STALLED_NP: anon_246 = 0;
pub const anon_246_PTHREAD_MUTEX_ROBUST: anon_246 = 1;
pub const anon_246_PTHREAD_MUTEX_ROBUST_NP: anon_246 = 1;
pub type anon_247 = u32;
pub const anon_247_PTHREAD_RWLOCK_PREFER_READER_NP: anon_247 = 0;
pub const anon_247_PTHREAD_RWLOCK_PREFER_WRITER_NP: anon_247 = 1;
pub const anon_247_PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP: anon_247 = 2;
pub const anon_247_PTHREAD_RWLOCK_DEFAULT_NP: anon_247 = 0;
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
pub type anon_248 = u32;
pub const anon_248_PTHREAD_CANCEL_DEFERRED: anon_248 = 0;
pub const anon_248_PTHREAD_CANCEL_ASYNCHRONOUS: anon_248 = 1;
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
    pub static __libc_single_threaded_249: Value<i8> = unimplemented!();
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
    pub static __default_lock_policy_250: Value<__gnu_cxx__Lock_policy> = unimplemented!();
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
pub struct _Mutex_base_251 {}
impl Clone for _Mutex_base_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_252 = u32;
pub const anon_252__S_need_barriers: anon_252 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Mutex_base_253 {}
impl Clone for _Mutex_base_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _Mutex_base_253 {}
impl _Mutex_base_253 {}
impl _Mutex_base_253 {
    pub fn copy_assign(&self, _a0: Ptr<_Mutex_base_253>) -> Ptr<_Mutex_base_253> {
        unimplemented!()
    }
}
impl _Mutex_base_253 {
    pub fn move_assign(&self, _a0: Ptr<_Mutex_base_253>) -> Ptr<_Mutex_base_253> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _Mutex_base_254 {}
impl Clone for _Mutex_base_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Mutex_base_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_255 = u32;
pub const anon_255__S_need_barriers: anon_255 = 0;
impl _Mutex_base_254 {}
impl _Mutex_base_254 {}
impl _Mutex_base_254 {
    pub fn copy_assign(&self, _a0: Ptr<_Mutex_base_254>) -> Ptr<_Mutex_base_254> {
        unimplemented!()
    }
}
impl _Mutex_base_254 {
    pub fn move_assign(&self, _a0: Ptr<_Mutex_base_254>) -> Ptr<_Mutex_base_254> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Sp_counted_ptr_256 {}
impl Clone for _Sp_counted_ptr_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(64)]
pub struct _Sp_counted_ptr_257 {}
impl Clone for _Sp_counted_ptr_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Sp_counted_ptr_258 {}
impl Clone for _Sp_counted_ptr_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_counted_ptr_258 {
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
pub struct _Sp_alloc_shared_tag_259 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_a: Ptr<allocator_0>,
}
impl Clone for _Sp_alloc_shared_tag_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_alloc_shared_tag_259 {
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
pub struct _Sp_owner_less_260 {}
impl Clone for _Sp_owner_less_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Sp_owner_less_260 {
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
pub fn change_261(n: Ptr<unique_ptr_230>) {
    let m: Value<unique_ptr_230> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(20));
            make_unique_262(___args.as_pointer())
        }),
    ));
    n.write({ (*m.borrow()).clone() });
    (if ({ unique_ptr_230Impl::get(&m.as_pointer()) }).is_null() {
        &(0);
    } else {
        ({
            __assert_fail_263(
                Ptr::<i8>::from_string_literal(b"m.get() == nullptr"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"void change(std::unique_ptr<int> &)"),
            )
        });
    });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let n: Value<unique_ptr_230> = Rc::new(RefCell::new(
        ({
            let ___args: Value<i32> = Rc::new(RefCell::new(10));
            make_unique_262(___args.as_pointer())
        }),
    ));
    ({ change_261(n.as_pointer()) });
    (if ((({ unique_ptr_230Impl::operator_deref(&n.as_pointer()) }).read()) == 20) {
        &(0);
    } else {
        ({
            __assert_fail_263(
                Ptr::<i8>::from_string_literal(b"*n == 20"),
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
pub struct __locale_data;
#[derive(Clone, Copy, Default, ByteRepr, VaArg, FnPtrArg)]
#[byte_size(1)]
pub struct std___uses_alloc0__Sink;
pub fn make_unique_262(__args: Ptr<i32>) -> unique_ptr_230 {
    unimplemented!()
}
pub fn __assert_fail_263(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
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
pub trait __uniq_ptr_impl_227Impl {
    fn move_assign(&self, __u: Ptr<__uniq_ptr_impl_227>) -> Ptr<__uniq_ptr_impl_227> {
        unimplemented!()
    }
    fn _M_ptr_6(&self) -> Ptr<Ptr<i32>> {
        unimplemented!()
    }
    fn _M_ptr_7(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deleter_8(&self) -> Ptr<default_delete_193> {
        unimplemented!()
    }
    fn reset(&self, mut __p: Ptr<i32>) {
        unimplemented!()
    }
    fn release(&self) -> Ptr<i32> {
        unimplemented!()
    }
}
impl __uniq_ptr_impl_227Impl for Ptr<__uniq_ptr_impl_227> {}
pub trait default_delete_193Impl {
    fn operator_call(&self, mut __ptr: Ptr<i32>) {
        unimplemented!()
    }
}
impl default_delete_193Impl for Ptr<default_delete_193> {}
pub trait integral_constant_4Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_4Impl for Ptr<integral_constant_4> {}
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
pub trait tuple_156Impl {
    fn copy_assign(&self, __in: Ptr<tuple_156>) -> Ptr<tuple_156> {
        unimplemented!()
    }
    fn move_assign(&self, __in: Ptr<tuple_156>) -> Ptr<tuple_156> {
        unimplemented!()
    }
}
impl tuple_156Impl for Ptr<tuple_156> {}
pub trait unique_ptr_230Impl {
    fn operator_deref(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn get(&self) -> Ptr<i32> {
        unimplemented!()
    }
    fn get_deleter_17(&self) -> Ptr<default_delete_193> {
        unimplemented!()
    }
}
impl unique_ptr_230Impl for Ptr<unique_ptr_230> {}
pub fn __cpp2rust_init_globals() {}
