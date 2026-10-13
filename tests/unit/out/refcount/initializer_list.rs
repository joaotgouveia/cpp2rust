extern crate libcc2rs;
use libcc2rs::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::prelude::*;
use std::io::{Read, Seek, Write};
use std::os::fd::AsFd;
use std::rc::{Rc, Weak};
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
pub type std_byte = u8;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_0 {}
impl Clone for initializer_list_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_1 {}
impl Clone for initializer_list_1 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_1 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl initializer_list_1 {}
impl initializer_list_1 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_2 {}
impl Clone for integral_constant_2 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_2 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__true__value_3: Value<bool> = unimplemented!();
);
impl integral_constant_2 {}
impl integral_constant_2 {}
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
    pub static std_integral_constant_bool__false__value_5: Value<bool> = unimplemented!();
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_7 {}
impl Clone for integral_constant_7 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_7 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_8 {}
impl Clone for enable_if_8 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_8 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_9 {}
impl Clone for enable_if_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_10 {}
impl Clone for __type_identity_10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_10 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_11 {}
impl Clone for __type_identity_11 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_11 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_12 {}
impl Clone for __type_identity_12 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_12 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_13 {}
impl Clone for __type_identity_13 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_13 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__14 {}
impl Clone for __or__14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__14 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__15 {}
impl Clone for __or__15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__15 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__16 {}
impl Clone for __or__16 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__16 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__17 {}
impl Clone for __or__17 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__17 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__18 {}
impl Clone for __or__18 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__18 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __or__19 {}
impl Clone for __or__19 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__19 {
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
pub struct __and__21 {}
impl Clone for __and__21 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__21 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_22 {}
impl Clone for is_reference_22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_22 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_23 {}
impl Clone for is_reference_23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_23 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_24 {}
impl Clone for is_reference_24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_24 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_25 {}
impl Clone for is_function_25 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_25 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_26 {}
impl Clone for is_function_26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_26 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_27 {}
impl Clone for is_function_27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_27 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_28 {}
impl Clone for is_void_28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_28 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_29 {}
impl Clone for is_void_29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_30 {}
impl Clone for is_void_30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_30 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_31 {}
impl Clone for is_void_31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_32 {}
impl Clone for remove_cv_32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_33 {}
impl Clone for remove_cv_33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_34 {}
impl Clone for remove_cv_34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_35 {}
impl Clone for remove_cv_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_36 {}
impl Clone for remove_cv_36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_37 {}
impl Clone for remove_cv_37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_38 {}
impl Clone for is_const_38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_39 {}
impl Clone for is_const_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_40 {}
impl Clone for is_const_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_41 {}
impl Clone for is_const_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_42 {}
impl Clone for is_const_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_43 {}
impl Clone for __is_array_unknown_bounds_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_44 {}
impl Clone for __is_array_unknown_bounds_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_45 {}
impl Clone for __is_array_unknown_bounds_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_46 {}
impl Clone for __is_integral_helper_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_47 {}
impl Clone for __is_integral_helper_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_48 {}
impl Clone for __is_integral_helper_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_49 {}
impl Clone for __is_integral_helper_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_50 {}
impl Clone for __is_integral_helper_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_51 {}
impl Clone for __is_integral_helper_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_52 {}
impl Clone for is_integral_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_53 {}
impl Clone for is_integral_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_54 {}
impl Clone for is_integral_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_55 {}
impl Clone for is_integral_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_56 {}
impl Clone for is_integral_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_57 {}
impl Clone for is_integral_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_58 {}
impl Clone for is_arithmetic_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_59 {}
impl Clone for is_scalar_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_60 {}
impl Clone for is_same_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_61 {}
impl Clone for is_volatile_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_62 {}
impl Clone for is_volatile_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_63 {}
impl Clone for is_volatile_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_64 {}
impl Clone for is_volatile_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_65 {}
impl Clone for is_volatile_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_65 {
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
pub struct __is_destructible_safe_66 {}
impl Clone for __is_destructible_safe_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_66 {
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
pub struct is_nothrow_copy_constructible_67 {}
impl Clone for is_nothrow_copy_constructible_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_68 {}
impl Clone for is_nothrow_copy_constructible_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_68 {
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
pub struct is_trivially_destructible_69 {}
impl Clone for is_trivially_destructible_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_70 {}
impl Clone for remove_reference_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_71 {}
impl Clone for __cv_selector_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_72 {}
impl Clone for __cv_selector_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_73 {}
impl Clone for __cv_selector_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_74 {}
impl Clone for __cv_selector_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_75 {}
impl Clone for __match_cv_qualifiers_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_76 {}
impl Clone for __match_cv_qualifiers_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_77 {}
impl Clone for __match_cv_qualifiers_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_78 {}
impl Clone for __match_cv_qualifiers_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_79 {}
impl Clone for __match_cv_qualifiers_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_80 {}
impl Clone for __make_unsigned_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_81 {}
impl Clone for __make_unsigned_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_82 {}
impl Clone for __make_unsigned_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_83 {}
impl Clone for __make_unsigned_selector_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_84 {}
impl Clone for __make_unsigned_selector_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_85 {}
impl Clone for __make_unsigned_selector_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_86 {}
impl Clone for __make_unsigned_selector_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_87 {}
impl Clone for __make_unsigned_selector_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_88 {}
impl Clone for __make_unsigned_selector_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_88 {
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
pub struct _List_92 {}
impl Clone for _List_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_93 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_94 {}
impl Clone for _List_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_94 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_95 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_96 {}
impl Clone for _List_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_96 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_97 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_98 {}
impl Clone for _List_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_98 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_99 {}
impl Clone for _List_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_100 {}
impl Clone for _List_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_101 {}
impl Clone for __select_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_102 {}
impl Clone for __select_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_103 {}
impl Clone for __select_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_104 {}
impl Clone for __select_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_104 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_105 {}
impl Clone for __select_105 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_105 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_106 {}
impl Clone for __make_signed_106 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_106 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_107 {}
impl Clone for __make_signed_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_108 {}
impl Clone for __make_signed_selector_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_109 {}
impl Clone for __make_signed_selector_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_109 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_110 {}
impl Clone for __make_signed_selector_110 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_110 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_111 {}
impl Clone for __make_signed_selector_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_112 {}
impl Clone for __make_signed_selector_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_pointer_113 {}
impl Clone for remove_pointer_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_pointer_113 {
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
pub struct common_type_114 {}
impl Clone for common_type_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_114 {
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
pub struct __are_same_115 {}
impl Clone for __are_same_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_116 = u32;
pub const anon_116___value: anon_116 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_117 {}
impl Clone for __are_same_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_118 = u32;
pub const anon_118___value: anon_118 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_119 {}
impl Clone for __are_same_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_120 = u32;
pub const anon_120___value: anon_120 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_121 {}
impl Clone for __are_same_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_122 = u32;
pub const anon_122___value: anon_122 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_123 {}
impl Clone for __are_same_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_124 = u32;
pub const anon_124___value: anon_124 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_125 {}
impl Clone for __are_same_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_126 = u32;
pub const anon_126___value: anon_126 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_127 {}
impl Clone for __is_integer_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_128 = u32;
pub const anon_128___value: anon_128 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_129 {}
impl Clone for __is_integer_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_130 = u32;
pub const anon_130___value: anon_130 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_131 {}
impl Clone for __is_integer_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_132 = u32;
pub const anon_132___value: anon_132 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_133 {}
impl Clone for __is_integer_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_134 = u32;
pub const anon_134___value: anon_134 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_135 {}
impl Clone for __is_integer_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_135 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_136 = u32;
pub const anon_136___value: anon_136 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nonvolatile_trivially_copyable_137 {}
impl Clone for __is_nonvolatile_trivially_copyable_137 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nonvolatile_trivially_copyable_137 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_138 = u32;
pub const anon_138___value: anon_138 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_139 {}
impl Clone for __memcpyable_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_140 {}
impl Clone for __memcpyable_integer_140 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_140 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_141 {}
impl Clone for __is_integer_nonstrict_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_142 = u32;
pub const anon_142___width: anon_142 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_143 {}
impl Clone for __is_integer_nonstrict_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_144 = u32;
pub const anon_144___width: anon_144 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_145 {}
impl Clone for __numeric_traits_integer_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_146: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_147: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_148: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_149 {}
impl Clone for __numeric_traits_integer_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_150: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_151: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_152: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_153 {}
impl Clone for __numeric_traits_floating_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_154 {}
impl Clone for __numeric_traits_floating_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_155 {}
impl Clone for __numeric_traits_floating_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_156 {}
impl Clone for __numeric_traits_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_156 {
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
    pub static piecewise_construct_157: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_158 {}
impl Clone for tuple_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_158 {}
impl tuple_158 {}
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
pub struct iterator_159 {}
impl Clone for iterator_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_160 {}
impl Clone for iterator_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_160 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_160 {}
impl iterator_160 {}
impl iterator_160 {}
impl iterator_160 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_160>) -> Ptr<iterator_160> {
        unimplemented!()
    }
}
impl iterator_160 {
    pub fn move_assign(&self, _a0: Ptr<iterator_160>) -> Ptr<iterator_160> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_161 {}
impl Clone for iterator_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_161 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_161>) -> Ptr<iterator_161> {
        unimplemented!()
    }
}
impl iterator_161 {
    pub fn move_assign(&self, _a0: Ptr<iterator_161>) -> Ptr<iterator_161> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_162 {}
impl Clone for iterator_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_162 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_162>) -> Ptr<iterator_162> {
        unimplemented!()
    }
}
impl iterator_162 {
    pub fn move_assign(&self, _a0: Ptr<iterator_162>) -> Ptr<iterator_162> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_163 {}
impl Clone for iterator_traits_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_164 {}
impl Clone for iterator_traits_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_164 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_165 {}
impl Clone for iterator_traits_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_166 {}
impl Clone for __iterator_traits_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_167 {}
impl Clone for __iterator_traits_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_168 {}
impl Clone for reverse_iterator_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_168 {
    pub fn copy_from(__x: Ptr<reverse_iterator_168>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_168 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_168>) -> Ptr<reverse_iterator_168> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_169 {}
impl Clone for reverse_iterator_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_169 {
    pub fn copy_from(__x: Ptr<reverse_iterator_169>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_169 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_169>) -> Ptr<reverse_iterator_169> {
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_170 {}
impl Clone for __new_allocator_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_170 {
    pub fn copy_from(_a0: Ptr<__new_allocator_170>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_170 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_170>) -> Ptr<__new_allocator_170> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_171 {}
impl Clone for __new_allocator_171 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_171 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_171 {
    pub fn copy_from(_a0: Ptr<__new_allocator_171>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_171 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_171>) -> Ptr<__new_allocator_171> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_172 {}
impl Clone for __new_allocator_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_172 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_172 {
    pub fn copy_from(_a0: Ptr<__new_allocator_172>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_172 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_172>) -> Ptr<__new_allocator_172> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_173 {}
impl Clone for allocator_173 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_173 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_173 {}
impl allocator_173 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_174 {}
impl Clone for allocator_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_174 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_174 {
    pub fn copy_from(__a: Ptr<allocator_174>) -> Self {
        unimplemented!()
    }
}
impl allocator_174 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_174>) -> Ptr<allocator_174> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_175 {}
impl Clone for allocator_traits_175 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_175 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_175 {
    pub fn allocate_1(__a: Ptr<allocator_174>, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl allocator_traits_175 {
    pub fn deallocate(__a: Ptr<allocator_174>, mut __p: Ptr<i32>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_175 {
    pub fn max_size(__a: Ptr<allocator_174>) -> u64 {
        unimplemented!()
    }
}
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
pub struct __alloc_traits_177 {}
impl Clone for __alloc_traits_177 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_177 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_178 {}
impl Clone for rebind_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_179 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_int__std_allocator_int____Vector_impl,
}
impl Clone for _Vector_base_179 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_179 {
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
    pub fn new_1(__a: Ptr<allocator_174>) -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_int__std_allocator_int____Vector_impl {
    pub fn move_from(__x: Ptr<std__Vector_base_int__std_allocator_int____Vector_impl>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_179 {
    pub fn new_1(__a: Ptr<allocator_174>) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_179 {
    pub fn move_from(_a0: Ptr<_Vector_base_179>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_180 {}
impl Clone for vector_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl vector_180 {
    pub fn copy_from(__x: Ptr<vector_180>) -> Self {
        unimplemented!()
    }
}
impl vector_180 {
    pub fn move_from(_a0: Ptr<vector_180>) -> Self {
        unimplemented!()
    }
}
impl vector_180 {
    pub fn new_1(__l: initializer_list_1, __a: Option<Ptr<allocator_174>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_181 {}
impl Clone for __hash_base_181 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_181 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_182 {}
impl Clone for __hash_base_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_183 {}
impl Clone for __hash_base_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_184 {}
impl Clone for __hash_base_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_185 {}
impl Clone for __hash_base_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_185 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_186 {}
impl Clone for __hash_base_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_187 {}
impl Clone for __hash_base_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_188 {}
impl Clone for __hash_base_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_189 {}
impl Clone for __hash_base_189 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_189 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_190 {}
impl Clone for __hash_base_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_191 {}
impl Clone for __hash_base_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_192 {}
impl Clone for __hash_base_192 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_192 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_193 {}
impl Clone for __hash_base_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_194 {}
impl Clone for __hash_base_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_195 {}
impl Clone for __hash_base_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_196 {}
impl Clone for __hash_base_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_197 {}
impl Clone for __hash_base_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_198 {}
impl Clone for __hash_base_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_199 {}
impl Clone for __hash_base_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_200 {}
impl Clone for __hash_base_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_201 {}
impl Clone for __hash_base_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_202 {}
impl Clone for hash_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_202 {
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
    pub fn hash_203(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_204(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_205 = u32;
pub const anon_205__S_word_bit: anon_205 = 64;
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct binary_function_206 {}
impl Clone for binary_function_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_206 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_207 {}
impl Clone for greater_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_208 {}
impl Clone for less_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_209 {}
impl Clone for greater_equal_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_210 {}
impl Clone for less_equal_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_210 {
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
    pub static allocator_arg_211: Value<std_allocator_arg_t> = unimplemented!();
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
pub fn f_212(bytes: initializer_list_1) -> usize {
    let bytes: Value<initializer_list_1> = Rc::new(RefCell::new(bytes));
    let mut buf: Ptr<vector_180> =
        Ptr::alloc(vector_180::new_1({ (*bytes.borrow()).clone() }, None));
    let mut n: usize = (({ initializer_list_1Impl::size(&bytes.as_pointer()) }) as usize);
    buf.delete();
    return n;
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    (if (({ f_212(__cpp2rust_unimplemented::<initializer_list_1>()) }) == 3_usize) {
        &(0);
    } else {
        ({
            __assert_fail_213(
                Ptr::<i8>::from_string_literal(b"f({1, 2, 3}) == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let v: Value<vector_180> = Rc::new(RefCell::new(vector_180::new_1(
        { __cpp2rust_unimplemented::<initializer_list_1>() },
        None,
    )));
    (if (({ vector_180Impl::size(&v.as_pointer()) }) == 3_u64) {
        &(0);
    } else {
        ({
            __assert_fail_213(
                Ptr::<i8>::from_string_literal(b"v.size() == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ((((({ vector_180Impl::operator_index_46(&v.as_pointer(), 0_u64) }).read())
        + (({ vector_180Impl::operator_index_46(&v.as_pointer(), 1_u64) }).read()))
        + (({ vector_180Impl::operator_index_46(&v.as_pointer(), 2_u64) }).read()))
        == 15)
    {
        &(0);
    } else {
        ({
            __assert_fail_213(
                Ptr::<i8>::from_string_literal(b"v[0] + v[1] + v[2] == 15"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let l: Value<initializer_list_1> = Rc::new(RefCell::new(__cpp2rust_unimplemented::<
        initializer_list_1,
    >()));
    (if (({ initializer_list_1Impl::size(&l.as_pointer()) }) == 2_u64) {
        &(0);
    } else {
        ({
            __assert_fail_213(
                Ptr::<i8>::from_string_literal(b"l.size() == 2"),
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
#[track_caller]
pub fn __cpp2rust_unimplemented<T>() -> T {
    unimplemented!()
}
pub fn __assert_fail_213(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait _Vector_base_179Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_174> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<i32> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<i32>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_179Impl for Ptr<_Vector_base_179> {}
pub trait __new_allocator_172Impl {
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
impl __new_allocator_172Impl for Ptr<__new_allocator_172> {}
pub trait initializer_list_0Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_0Impl for Ptr<initializer_list_0> {}
pub trait initializer_list_1Impl {
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
impl initializer_list_1Impl for Ptr<initializer_list_1> {}
pub trait integral_constant_2Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_2Impl for Ptr<integral_constant_2> {}
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
pub trait vector_180Impl {
    fn copy_assign(&self, __x: Ptr<vector_180>) -> Ptr<vector_180> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_180>) -> Ptr<vector_180> {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<i32> {
        unimplemented!()
    }
}
impl vector_180Impl for Ptr<vector_180> {}
pub fn __cpp2rust_init_globals() {}
