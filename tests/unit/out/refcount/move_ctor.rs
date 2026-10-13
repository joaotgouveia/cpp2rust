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
pub struct integral_constant_0 {}
impl Clone for integral_constant_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_0 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std_integral_constant_bool__true__value_1: Value<bool> = unimplemented!();
);
impl integral_constant_0 {}
impl integral_constant_0 {}
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
    pub static std_integral_constant_bool__false__value_3: Value<bool> = unimplemented!();
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
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct integral_constant_5 {}
impl Clone for integral_constant_5 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for integral_constant_5 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_6 {}
impl Clone for enable_if_6 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_6 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct enable_if_7 {}
impl Clone for enable_if_7 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for enable_if_7 {
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
pub struct __conditional_9 {}
impl Clone for __conditional_9 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_9 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __conditional_10 {}
impl Clone for __conditional_10 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __conditional_10 {
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
pub struct __type_identity_14 {}
impl Clone for __type_identity_14 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_14 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __type_identity_15 {}
impl Clone for __type_identity_15 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __type_identity_15 {
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
pub struct __or__24 {}
impl Clone for __or__24 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __or__24 {
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
pub struct __not__31 {}
impl Clone for __not__31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__31 {
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
pub struct __not__34 {}
impl Clone for __not__34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__34 {
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
pub struct is_reference_38 {}
impl Clone for is_reference_38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_39 {}
impl Clone for is_reference_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_40 {}
impl Clone for is_reference_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_41 {}
impl Clone for is_function_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_42 {}
impl Clone for is_function_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_43 {}
impl Clone for is_function_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_44 {}
impl Clone for is_function_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_45 {}
impl Clone for is_function_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_46 {}
impl Clone for is_void_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_47 {}
impl Clone for is_void_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_48 {}
impl Clone for is_void_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_49 {}
impl Clone for is_void_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_50 {}
impl Clone for is_void_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_51 {}
impl Clone for is_void_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_52 {}
impl Clone for remove_cv_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_53 {}
impl Clone for remove_cv_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_54 {}
impl Clone for remove_cv_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_55 {}
impl Clone for remove_cv_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_55 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_56 {}
impl Clone for remove_cv_56 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_56 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_57 {}
impl Clone for remove_cv_57 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_57 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_58 {}
impl Clone for is_const_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_59 {}
impl Clone for is_const_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_60 {}
impl Clone for is_const_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_60 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_61 {}
impl Clone for is_const_61 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_61 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_62 {}
impl Clone for is_const_62 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_62 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_63 {}
impl Clone for __is_array_unknown_bounds_63 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_63 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_64 {}
impl Clone for __is_array_unknown_bounds_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_65 {}
impl Clone for __is_array_unknown_bounds_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_65 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_66 {}
impl Clone for __is_array_unknown_bounds_66 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_66 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_67 {}
impl Clone for __is_array_unknown_bounds_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_68 {}
impl Clone for __is_integral_helper_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_69 {}
impl Clone for __is_integral_helper_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_70 {}
impl Clone for __is_integral_helper_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_71 {}
impl Clone for __is_integral_helper_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_72 {}
impl Clone for __is_integral_helper_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_73 {}
impl Clone for __is_integral_helper_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_74 {}
impl Clone for is_integral_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_75 {}
impl Clone for is_integral_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_76 {}
impl Clone for is_integral_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_77 {}
impl Clone for is_integral_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_78 {}
impl Clone for is_integral_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_79 {}
impl Clone for is_integral_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_floating_point_helper_80 {}
impl Clone for __is_floating_point_helper_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_80 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_81 {}
impl Clone for is_floating_point_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_82 {}
impl Clone for is_pointer_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_82 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_pointer_83 {}
impl Clone for is_pointer_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_84 {}
impl Clone for is_enum_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_null_pointer_85 {}
impl Clone for is_null_pointer_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_86 {}
impl Clone for is_arithmetic_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_pointer_87 {}
impl Clone for is_member_pointer_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_88 {}
impl Clone for is_scalar_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_89 {}
impl Clone for is_same_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_90 {}
impl Clone for is_same_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_91 {}
impl Clone for is_volatile_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_91 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_92 {}
impl Clone for is_volatile_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_93 {}
impl Clone for is_volatile_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_93 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_94 {}
impl Clone for is_volatile_94 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_94 {
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
pub struct remove_all_extents_96 {}
impl Clone for remove_all_extents_96 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_96 {
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
pub struct __is_destructible_impl_97 {}
impl Clone for __is_destructible_impl_97 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_97 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_98 {}
impl Clone for __is_destructible_safe_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_98 {
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
pub struct __is_nt_destructible_impl_99 {}
impl Clone for __is_nt_destructible_impl_99 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_impl_99 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_safe_100 {}
impl Clone for __is_nt_destructible_safe_100 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_100 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_101 {}
impl Clone for is_nothrow_destructible_101 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_101 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_102 {}
impl Clone for is_constructible_102 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_102 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_constructible_103 {}
impl Clone for is_copy_constructible_103 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_constructible_103 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_constructible_104 {}
impl Clone for is_copy_constructible_104 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_constructible_104 {
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
pub struct is_nothrow_copy_constructible_107 {}
impl Clone for is_nothrow_copy_constructible_107 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_107 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_108 {}
impl Clone for is_nothrow_move_constructible_108 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_108 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_109 {}
impl Clone for is_nothrow_move_constructible_109 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_109 {
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
pub struct is_trivially_destructible_111 {}
impl Clone for is_trivially_destructible_111 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_111 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_112 {}
impl Clone for __is_nothrow_new_constructible_impl_112 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_112 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_113 {}
impl Clone for remove_reference_113 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_113 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_114 {}
impl Clone for remove_reference_114 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_114 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_115 {}
impl Clone for remove_reference_115 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_115 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_116 {}
impl Clone for remove_reference_116 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_116 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_reference_117 {}
impl Clone for remove_reference_117 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_117 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_118 {}
impl Clone for __cv_selector_118 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_118 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_119 {}
impl Clone for __cv_selector_119 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_119 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_120 {}
impl Clone for __cv_selector_120 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_120 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __cv_selector_121 {}
impl Clone for __cv_selector_121 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __cv_selector_121 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_122 {}
impl Clone for __match_cv_qualifiers_122 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_122 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_123 {}
impl Clone for __match_cv_qualifiers_123 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_123 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_124 {}
impl Clone for __match_cv_qualifiers_124 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_124 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_125 {}
impl Clone for __match_cv_qualifiers_125 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_125 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __match_cv_qualifiers_126 {}
impl Clone for __match_cv_qualifiers_126 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __match_cv_qualifiers_126 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_127 {}
impl Clone for __make_unsigned_127 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_127 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_128 {}
impl Clone for __make_unsigned_128 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_128 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_129 {}
impl Clone for __make_unsigned_129 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_129 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_130 {}
impl Clone for __make_unsigned_selector_130 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_130 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_131 {}
impl Clone for __make_unsigned_selector_131 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_131 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_132 {}
impl Clone for __make_unsigned_selector_132 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_132 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_133 {}
impl Clone for __make_unsigned_selector_133 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_133 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_134 {}
impl Clone for __make_unsigned_selector_134 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_134 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_unsigned_selector_135 {}
impl Clone for __make_unsigned_selector_135 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_unsigned_selector_135 {
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
pub struct _List_139 {}
impl Clone for _List_139 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_139 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_char__unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_140 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_141 {}
impl Clone for _List_141 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_141 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_short__unsigned_int__unsigned_long__unsigned_long_long____size_142 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_143 {}
impl Clone for _List_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_143 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static std___make_unsigned_selector_base__List_unsigned_int__unsigned_long__unsigned_long_long____size_144 : Value<usize > = unimplemented!() ;
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_145 {}
impl Clone for _List_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_146 {}
impl Clone for _List_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_146 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct _List_147 {}
impl Clone for _List_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _List_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_148 {}
impl Clone for __select_148 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_148 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_149 {}
impl Clone for __select_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_150 {}
impl Clone for __select_150 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_150 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_151 {}
impl Clone for __select_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __select_152 {}
impl Clone for __select_152 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __select_152 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_153 {}
impl Clone for __make_signed_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_154 {}
impl Clone for __make_signed_154 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_154 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_155 {}
impl Clone for __make_signed_selector_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_156 {}
impl Clone for __make_signed_selector_156 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_156 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_157 {}
impl Clone for __make_signed_selector_157 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_157 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_158 {}
impl Clone for __make_signed_selector_158 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_158 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __make_signed_selector_159 {}
impl Clone for __make_signed_selector_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __make_signed_selector_159 {
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
pub struct common_type_160 {}
impl Clone for common_type_160 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for common_type_160 {
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
thread_local!(
    pub static __is_nothrow_new_constructible_161: Value<bool> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __move_if_noexcept_cond_162 {}
impl Clone for __move_if_noexcept_cond_162 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __move_if_noexcept_cond_162 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __move_if_noexcept_cond_163 {}
impl Clone for __move_if_noexcept_cond_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __move_if_noexcept_cond_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __move_if_noexcept_cond_164 {}
impl Clone for __move_if_noexcept_cond_164 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __move_if_noexcept_cond_164 {
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
    pub static piecewise_construct_165: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_166 {}
impl Clone for tuple_166 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_166 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_166 {}
impl tuple_166 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_167 {}
impl Clone for initializer_list_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_167 {
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
pub struct __are_same_168 {}
impl Clone for __are_same_168 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_168 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_169 = u32;
pub const anon_169___value: anon_169 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_170 {}
impl Clone for __are_same_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_171 = u32;
pub const anon_171___value: anon_171 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_172 {}
impl Clone for __are_same_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_173 = u32;
pub const anon_173___value: anon_173 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_174 {}
impl Clone for __are_same_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_175 = u32;
pub const anon_175___value: anon_175 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_176 {}
impl Clone for __are_same_176 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_176 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_177 = u32;
pub const anon_177___value: anon_177 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_178 {}
impl Clone for __are_same_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_179 = u32;
pub const anon_179___value: anon_179 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_180 {}
impl Clone for __is_integer_180 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_180 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_181 = u32;
pub const anon_181___value: anon_181 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_182 {}
impl Clone for __is_integer_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_183 = u32;
pub const anon_183___value: anon_183 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_184 {}
impl Clone for __is_integer_184 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_184 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_185 = u32;
pub const anon_185___value: anon_185 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_186 {}
impl Clone for __is_integer_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_187 = u32;
pub const anon_187___value: anon_187 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_188 {}
impl Clone for __is_integer_188 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_188 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_189 = u32;
pub const anon_189___value: anon_189 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_190 {}
impl Clone for __memcpyable_integer_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_191 {}
impl Clone for __is_integer_nonstrict_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_191 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_192 = u32;
pub const anon_192___width: anon_192 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_193 {}
impl Clone for __is_integer_nonstrict_193 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_193 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_194 = u32;
pub const anon_194___width: anon_194 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_195 {}
impl Clone for __numeric_traits_integer_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_196: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_197: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_198: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_199 {}
impl Clone for __numeric_traits_integer_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_200: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_201: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_202: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_203 {}
impl Clone for __numeric_traits_floating_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_204 {}
impl Clone for __numeric_traits_floating_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_205 {}
impl Clone for __numeric_traits_floating_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_206 {}
impl Clone for __numeric_traits_206 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_206 {
    fn default() -> Self {
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
pub struct iterator_207 {}
impl Clone for iterator_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_208 {}
impl Clone for iterator_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_208 {}
impl iterator_208 {}
impl iterator_208 {}
impl iterator_208 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_208>) -> Ptr<iterator_208> {
        unimplemented!()
    }
}
impl iterator_208 {
    pub fn move_assign(&self, _a0: Ptr<iterator_208>) -> Ptr<iterator_208> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_209 {}
impl Clone for iterator_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_209 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_209>) -> Ptr<iterator_209> {
        unimplemented!()
    }
}
impl iterator_209 {
    pub fn move_assign(&self, _a0: Ptr<iterator_209>) -> Ptr<iterator_209> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_210 {}
impl Clone for iterator_210 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_210 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_210 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_210>) -> Ptr<iterator_210> {
        unimplemented!()
    }
}
impl iterator_210 {
    pub fn move_assign(&self, _a0: Ptr<iterator_210>) -> Ptr<iterator_210> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_211 {}
impl Clone for iterator_traits_211 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_211 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_212 {}
impl Clone for iterator_traits_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_213 {}
impl Clone for iterator_traits_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_214 {}
impl Clone for iterator_traits_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_215 {}
impl Clone for __iterator_traits_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_216 {}
impl Clone for __iterator_traits_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_217 {}
impl Clone for __ptr_traits_ptr_to_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_217 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_218 {}
impl Clone for pointer_traits_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_218 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_219 {}
impl Clone for reverse_iterator_219 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_219 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_219 {
    pub fn copy_from(__x: Ptr<reverse_iterator_219>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_219 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_219>) -> Ptr<reverse_iterator_219> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_220 {}
impl Clone for reverse_iterator_220 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_220 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_220 {
    pub fn copy_from(__x: Ptr<reverse_iterator_220>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_220 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_220>) -> Ptr<reverse_iterator_220> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_221 {}
impl Clone for __normal_iterator_221 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_221 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_221 {
    pub fn new_1(__i: Ptr<Ptr<MoveOnly>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_221 {}
impl __normal_iterator_221 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_222 {}
impl Clone for __normal_iterator_222 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_222 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_222 {
    pub fn new_1(__i: Ptr<Ptr<MoveOnly>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_222 {}
impl __normal_iterator_222 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct move_iterator_223 {}
impl Clone for move_iterator_223 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for move_iterator_223 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl move_iterator_223 {
    pub fn new_1(__i: Ptr<MoveOnly>) -> Self {
        unimplemented!()
    }
}
impl move_iterator_223 {}
impl move_iterator_223 {}
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
pub struct __new_allocator_224 {}
impl Clone for __new_allocator_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_224 {
    pub fn copy_from(_a0: Ptr<__new_allocator_224>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_224 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_224>) -> Ptr<__new_allocator_224> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_225 {}
impl Clone for __new_allocator_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_225 {
    pub fn copy_from(_a0: Ptr<__new_allocator_225>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_225 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_225>) -> Ptr<__new_allocator_225> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_226 {}
impl Clone for __new_allocator_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_226 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_226 {
    pub fn copy_from(_a0: Ptr<__new_allocator_226>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_226 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_226>) -> Ptr<__new_allocator_226> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_228 {}
impl Clone for allocator_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_228 {}
impl allocator_228 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_229 {}
impl Clone for allocator_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_229 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_229 {
    pub fn copy_from(__a: Ptr<allocator_229>) -> Self {
        unimplemented!()
    }
}
impl allocator_229 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_229>) -> Ptr<allocator_229> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_230 {}
impl Clone for allocator_traits_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_230 {
    pub fn allocate_1(__a: Ptr<allocator_229>, mut __n: u64) -> Ptr<MoveOnly> {
        unimplemented!()
    }
}
impl allocator_traits_230 {
    pub fn deallocate(__a: Ptr<allocator_229>, mut __p: Ptr<MoveOnly>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_230 {
    pub fn max_size(__a: Ptr<allocator_229>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_230 {
    pub fn construct_231(__a: Ptr<allocator_229>, mut __p: Ptr<MoveOnly>, __args: Ptr<MoveOnly>) {
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
pub struct __is_move_insertable_233 {}
impl Clone for __is_move_insertable_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_234 {}
impl Clone for __alloc_traits_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_235 {}
impl Clone for rebind_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _UninitDestroyGuard_236 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_first: Ptr<MoveOnly>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_cur: Ptr<Ptr<MoveOnly>>,
}
impl Clone for _UninitDestroyGuard_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_236 {
    pub fn new(__first: Ptr<Ptr<MoveOnly>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_237 {}
impl Clone for __is_bitwise_relocatable_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_238 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl,
}
impl Clone for _Vector_base_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl_data {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_start: Ptr<MoveOnly>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_finish: Ptr<MoveOnly>,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_end_of_storage: Ptr<MoveOnly>,
}
impl Clone for std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl_data {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl_data {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl_data {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl_data {
    pub fn move_from(
        __x: Ptr<std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl_data>,
    ) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl {}
impl Clone for std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl {
    pub fn move_from(
        __x: Ptr<std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl>,
    ) -> Self {
        unimplemented!()
    }
}
impl _Vector_base_238 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_238 {
    pub fn move_from(_a0: Ptr<_Vector_base_238>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_239 {}
impl Clone for vector_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct std_vector_MoveOnly___Guard_alloc {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_storage: Ptr<MoveOnly>,
    #[offset(8)]
    pub _M_len: u64,
    #[offset(16)]
    #[byte_size(8)]
    pub _M_vect: Ptr<_Vector_base_238>,
}
impl Clone for std_vector_MoveOnly___Guard_alloc {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for std_vector_MoveOnly___Guard_alloc {
    fn default() -> Self {
        unimplemented!()
    }
}
impl std_vector_MoveOnly___Guard_alloc {
    pub fn new(mut __s: Ptr<MoveOnly>, mut __l: u64, __vect: Ptr<_Vector_base_238>) -> Self {
        unimplemented!()
    }
}
impl vector_239 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_239 {
    pub fn copy_from(__x: Ptr<vector_239>) -> Self {
        unimplemented!()
    }
}
impl vector_239 {
    pub fn move_from(_a0: Ptr<vector_239>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_241 {}
impl Clone for __hash_base_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_241 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_242 {}
impl Clone for __hash_base_242 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_242 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_243 {}
impl Clone for __hash_base_243 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_243 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_244 {}
impl Clone for __hash_base_244 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_244 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_245 {}
impl Clone for __hash_base_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_246 {}
impl Clone for __hash_base_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_247 {}
impl Clone for __hash_base_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_248 {}
impl Clone for __hash_base_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_249 {}
impl Clone for __hash_base_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_249 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_250 {}
impl Clone for __hash_base_250 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_250 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_251 {}
impl Clone for __hash_base_251 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_251 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_252 {}
impl Clone for __hash_base_252 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_252 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_253 {}
impl Clone for __hash_base_253 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_253 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_254 {}
impl Clone for __hash_base_254 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_254 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_255 {}
impl Clone for __hash_base_255 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_255 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_256 {}
impl Clone for __hash_base_256 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_256 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_257 {}
impl Clone for __hash_base_257 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_257 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_258 {}
impl Clone for __hash_base_258 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_258 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_259 {}
impl Clone for __hash_base_259 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_259 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_260 {}
impl Clone for __hash_base_260 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_260 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_261 {}
impl Clone for __hash_base_261 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_261 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_262 {}
impl Clone for hash_262 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_262 {
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
    pub fn hash_263(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_264(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_265 = u32;
pub const anon_265__S_word_bit: anon_265 = 64;
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
pub struct binary_function_266 {}
impl Clone for binary_function_266 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_266 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_267 {}
impl Clone for greater_267 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_267 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_268 {}
impl Clone for less_268 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_268 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_269 {}
impl Clone for greater_equal_269 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_269 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_270 {}
impl Clone for less_equal_270 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_270 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
    pub static allocator_arg_271: Value<std_allocator_arg_t> = unimplemented!();
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
#[derive(Record, ByteRepr, MoveCtor, FnPtrArg, Default)]
#[byte_size(4)]
pub struct MoveOnly {
    #[offset(0)]
    pub v: i32,
}
impl MoveOnly {
    pub fn new(mut v: i32) -> Self {
        Self { v: v }
    }
    pub fn move_from(o: Ptr<MoveOnly>) -> Self {
        let __this: MoveOnly = Self {
            v: o.with(|__s| __s.v),
        };
        field!(o, v).write(0);
        __this
    }
}
#[derive(Record, ByteRepr, FnPtrArg)]
#[byte_size(4)]
pub struct ConstMove {
    #[offset(0)]
    pub mark: i32,
}
impl ConstMove {
    pub fn new() -> Self {
        Self { mark: 0 }
    }
    pub fn new_1(o: Ptr<ConstMove>) -> Self {
        Self {
            mark: (o.with(|__s| __s.mark) + 1),
        }
    }
    pub fn new_2(o: Ptr<ConstMove>) -> Self {
        Self {
            mark: (o.with(|__s| __s.mark) + 10),
        }
    }
}
impl Default for ConstMove {
    fn default() -> Self {
        { ConstMove::new() }
    }
}
#[derive(Record, ByteRepr, VaArg, MoveCtor, FnPtrArg, Default)]
#[byte_size(12)]
pub struct ThrowingMove {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub copies: i32,
    #[offset(8)]
    pub moves: i32,
}
impl ThrowingMove {
    pub fn new(mut v: i32) -> Self {
        Self {
            v: v,
            copies: 0,
            moves: 0,
        }
    }
    pub fn copy_from(o: Ptr<ThrowingMove>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
            copies: (o.with(|__s| __s.copies) + 1),
            moves: o.with(|__s| __s.moves),
        }
    }
    pub fn move_from(o: Ptr<ThrowingMove>) -> Self {
        let __this: ThrowingMove = Self {
            v: o.with(|__s| __s.v),
            copies: o.with(|__s| __s.copies),
            moves: (o.with(|__s| __s.moves) + 1),
        };
        field!(o, v).write(0);
        __this
    }
}
impl Clone for ThrowingMove {
    fn clone(&self) -> Self {
        let __src: Value<ThrowingMove> = Rc::new(RefCell::new(ThrowingMove {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        ThrowingMove::copy_from(__src.as_pointer())
    }
}
#[derive(Record, ByteRepr, VaArg, MoveCtor, FnPtrArg, Default)]
#[byte_size(12)]
pub struct NoexceptMove {
    #[offset(0)]
    pub v: i32,
    #[offset(4)]
    pub copies: i32,
    #[offset(8)]
    pub moves: i32,
}
impl NoexceptMove {
    pub fn new(mut v: i32) -> Self {
        Self {
            v: v,
            copies: 0,
            moves: 0,
        }
    }
    pub fn copy_from(o: Ptr<NoexceptMove>) -> Self {
        Self {
            v: o.with(|__s| __s.v),
            copies: (o.with(|__s| __s.copies) + 1),
            moves: o.with(|__s| __s.moves),
        }
    }
    pub fn move_from(o: Ptr<NoexceptMove>) -> Self {
        let __this: NoexceptMove = Self {
            v: o.with(|__s| __s.v),
            copies: o.with(|__s| __s.copies),
            moves: (o.with(|__s| __s.moves) + 1),
        };
        field!(o, v).write(0);
        __this
    }
}
impl Clone for NoexceptMove {
    fn clone(&self) -> Self {
        let __src: Value<NoexceptMove> = Rc::new(RefCell::new(NoexceptMove {
            v: self.v.clone(),
            copies: self.copies.clone(),
            moves: self.moves.clone(),
        }));
        NoexceptMove::copy_from(__src.as_pointer())
    }
}
pub fn by_value_272(mut m: MoveOnly) -> i32 {
    return m.v;
}
pub fn make_273(mut v: i32) -> MoveOnly {
    let m: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ v })));
    return MoveOnly::move_from({ m.as_pointer() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 1 })));
    let b: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::move_from({ a.as_pointer() })));
    (if ({ (*b.borrow()).v } == 1) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"b.v == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*a.borrow()).v } == 0) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"a.v == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let c: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::move_from({ b.as_pointer() })));
    (if ({ (*c.borrow()).v } == 1) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"c.v == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*b.borrow()).v } == 0) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"b.v == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let mut d: MoveOnly = MoveOnly::move_from({ c.as_pointer() });
    (if (d.v == 1) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"d.v == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*c.borrow()).v } == 0) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"c.v == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let e: Value<MoveOnly> = Rc::new(RefCell::new(({ make_273(5) })));
    (if ({ (*e.borrow()).v } == 5) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"e.v == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ by_value_272(MoveOnly::new({ 6 })) }) == 6) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"by_value(MoveOnly(6)) == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (({ by_value_272(MoveOnly::move_from({ e.as_pointer() })) }) == 5) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"by_value(std::move(e)) == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*e.borrow()).v } == 0) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"e.v == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let vec_: Value<vector_239> = Rc::new(RefCell::new(<vector_239>::default()));
    ({
        let ___x: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 7 })));
        vector_239Impl::push_back_58(&vec_.as_pointer(), ___x.as_pointer())
    });
    let f: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 8 })));
    ({ vector_239Impl::push_back_58(&vec_.as_pointer(), f.as_pointer()) });
    (if (({ vector_239Impl::operator_index_46(&vec_.as_pointer(), 0_u64) }).with(|__s| __s.v) == 7)
        && (({ vector_239Impl::operator_index_46(&vec_.as_pointer(), 1_u64) }).with(|__s| __s.v)
            == 8)
    {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"vec[0].v == 7 && vec[1].v == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*f.borrow()).v } == 0) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"f.v == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let m: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::new()));
    let mut m1: ConstMove = ConstMove::new_1({ m.as_pointer() });
    let cm: Value<ConstMove> = Rc::new(RefCell::new(ConstMove::new()));
    let mut m2: ConstMove = ConstMove::new_2({ cm.as_pointer() });
    (if (m1.mark == 1) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"m1.mark == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (m2.mark == 10) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"m2.mark == 10"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let t: Value<ThrowingMove> = Rc::new(RefCell::new(ThrowingMove::new({ 1 })));
    let mut t1: ThrowingMove = ThrowingMove::copy_from({ t.as_pointer() });
    (if (t1.v == 1) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"t1.v == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (t1.copies == 1) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"t1.copies == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (t1.moves == 0) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"t1.moves == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*t.borrow()).v } == 1) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"t.v == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let n: Value<NoexceptMove> = Rc::new(RefCell::new(NoexceptMove::new({ 2 })));
    let mut n1: NoexceptMove = NoexceptMove::move_from({ n.as_pointer() });
    (if (n1.v == 2) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"n1.v == 2"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (n1.copies == 0) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"n1.copies == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if (n1.moves == 1) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"n1.moves == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*n.borrow()).v } == 0) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"n.v == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let g: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 3 })));
    let mut g1: MoveOnly = MoveOnly::move_from({ g.as_pointer() });
    (if (g1.v == 3) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"g1.v == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*g.borrow()).v } == 0) {
        &(0);
    } else {
        ({
            __assert_fail_274(
                Ptr::<i8>::from_string_literal(b"g.v == 0"),
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
pub fn __assert_fail_274(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait _UninitDestroyGuard_236Impl {
    fn release(&self) {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_236Impl for Ptr<_UninitDestroyGuard_236> {}
pub trait _Vector_base_238Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_229> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_229> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<MoveOnly>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_238Impl for Ptr<_Vector_base_238> {}
pub trait __new_allocator_226Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<MoveOnly>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_227(&self, mut __p: Ptr<MoveOnly>, __args: Ptr<MoveOnly>) {
        unimplemented!()
    }
}
impl __new_allocator_226Impl for Ptr<__new_allocator_226> {}
pub trait __normal_iterator_221Impl {
    fn operator_deref(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_221 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<MoveOnly>> {
        unimplemented!()
    }
}
impl __normal_iterator_221Impl for Ptr<__normal_iterator_221> {}
pub trait __normal_iterator_222Impl {
    fn base(&self) -> Ptr<Ptr<MoveOnly>> {
        unimplemented!()
    }
}
impl __normal_iterator_222Impl for Ptr<__normal_iterator_222> {}
pub trait initializer_list_167Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_167Impl for Ptr<initializer_list_167> {}
pub trait integral_constant_0Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_0Impl for Ptr<integral_constant_0> {}
pub trait move_iterator_223Impl {
    fn base(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<move_iterator_223> {
        unimplemented!()
    }
}
impl move_iterator_223Impl for Ptr<move_iterator_223> {}
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
pub trait vector_239Impl {
    fn copy_assign(&self, __x: Ptr<vector_239>) -> Ptr<vector_239> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_239>) -> Ptr<vector_239> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_221 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_222 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_221 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_222 {
        unimplemented!()
    }
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn empty(&self) -> bool {
        unimplemented!()
    }
    fn operator_index_46(&self, mut __n: u64) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn back_53(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn push_back_58(&self, __x: Ptr<MoveOnly>) {
        unimplemented!()
    }
    fn emplace_back_240(&self, __args: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        unimplemented!()
    }
}
impl vector_239Impl for Ptr<vector_239> {}
pub fn __cpp2rust_init_globals() {}
