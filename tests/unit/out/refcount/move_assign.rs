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
pub struct __and__22 {}
impl Clone for __and__22 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__22 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __and__23 {}
impl Clone for __and__23 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __and__23 {
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
pub struct __not__25 {}
impl Clone for __not__25 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__25 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __not__26 {}
impl Clone for __not__26 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __not__26 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_27 {}
impl Clone for is_reference_27 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_27 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_28 {}
impl Clone for is_reference_28 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_28 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_29 {}
impl Clone for is_reference_29 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_29 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_reference_30 {}
impl Clone for is_reference_30 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_reference_30 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_31 {}
impl Clone for is_function_31 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_31 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_32 {}
impl Clone for is_function_32 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_32 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_function_33 {}
impl Clone for is_function_33 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_function_33 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_34 {}
impl Clone for is_void_34 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_34 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_35 {}
impl Clone for is_void_35 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_35 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_36 {}
impl Clone for is_void_36 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_36 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_void_37 {}
impl Clone for is_void_37 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_void_37 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_38 {}
impl Clone for remove_cv_38 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_38 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_39 {}
impl Clone for remove_cv_39 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_39 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_40 {}
impl Clone for remove_cv_40 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_40 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_41 {}
impl Clone for remove_cv_41 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_41 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_42 {}
impl Clone for remove_cv_42 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_42 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_cv_43 {}
impl Clone for remove_cv_43 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_cv_43 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_44 {}
impl Clone for is_const_44 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_44 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_45 {}
impl Clone for is_const_45 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_45 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_46 {}
impl Clone for is_const_46 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_46 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_47 {}
impl Clone for is_const_47 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_47 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_const_48 {}
impl Clone for is_const_48 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_const_48 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_49 {}
impl Clone for __is_array_unknown_bounds_49 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_49 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_50 {}
impl Clone for __is_array_unknown_bounds_50 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_50 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_array_unknown_bounds_51 {}
impl Clone for __is_array_unknown_bounds_51 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_array_unknown_bounds_51 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_52 {}
impl Clone for __is_integral_helper_52 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_52 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_53 {}
impl Clone for __is_integral_helper_53 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_53 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_54 {}
impl Clone for __is_integral_helper_54 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_54 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integral_helper_55 {}
impl Clone for __is_integral_helper_55 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integral_helper_55 {
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
pub struct is_integral_58 {}
impl Clone for is_integral_58 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_58 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_59 {}
impl Clone for is_integral_59 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_59 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_integral_60 {}
impl Clone for is_integral_60 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_integral_60 {
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
pub struct __is_floating_point_helper_64 {}
impl Clone for __is_floating_point_helper_64 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_floating_point_helper_64 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_floating_point_65 {}
impl Clone for is_floating_point_65 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_floating_point_65 {
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
pub struct is_pointer_67 {}
impl Clone for is_pointer_67 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_pointer_67 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_enum_68 {}
impl Clone for is_enum_68 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_enum_68 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_null_pointer_69 {}
impl Clone for is_null_pointer_69 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_null_pointer_69 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_arithmetic_70 {}
impl Clone for is_arithmetic_70 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_arithmetic_70 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_member_pointer_71 {}
impl Clone for is_member_pointer_71 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_member_pointer_71 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_scalar_72 {}
impl Clone for is_scalar_72 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_scalar_72 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_73 {}
impl Clone for is_same_73 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_73 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_same_74 {}
impl Clone for is_same_74 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_same_74 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_75 {}
impl Clone for is_volatile_75 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_75 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_76 {}
impl Clone for is_volatile_76 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_76 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_77 {}
impl Clone for is_volatile_77 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_77 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_78 {}
impl Clone for is_volatile_78 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_78 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_volatile_79 {}
impl Clone for is_volatile_79 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_volatile_79 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct remove_all_extents_80 {}
impl Clone for remove_all_extents_80 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_all_extents_80 {
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
pub struct __is_destructible_impl_81 {}
impl Clone for __is_destructible_impl_81 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_impl_81 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_destructible_safe_82 {}
impl Clone for __is_destructible_safe_82 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_destructible_safe_82 {
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
pub struct __is_nt_destructible_impl_83 {}
impl Clone for __is_nt_destructible_impl_83 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_impl_83 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nt_destructible_safe_84 {}
impl Clone for __is_nt_destructible_safe_84 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nt_destructible_safe_84 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_destructible_85 {}
impl Clone for is_nothrow_destructible_85 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_destructible_85 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_constructible_86 {}
impl Clone for is_constructible_86 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_constructible_86 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_copy_constructible_87 {}
impl Clone for is_copy_constructible_87 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_copy_constructible_87 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_move_constructible_88 {}
impl Clone for is_move_constructible_88 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_move_constructible_88 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_default_constructible_89 {}
impl Clone for is_nothrow_default_constructible_89 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_default_constructible_89 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_copy_constructible_90 {}
impl Clone for is_nothrow_copy_constructible_90 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_copy_constructible_90 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct is_nothrow_move_constructible_91 {}
impl Clone for is_nothrow_move_constructible_91 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_nothrow_move_constructible_91 {
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
pub struct is_trivially_destructible_92 {}
impl Clone for is_trivially_destructible_92 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for is_trivially_destructible_92 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_nothrow_new_constructible_impl_93 {}
impl Clone for __is_nothrow_new_constructible_impl_93 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_nothrow_new_constructible_impl_93 {
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
pub struct remove_reference_98 {}
impl Clone for remove_reference_98 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for remove_reference_98 {
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
    pub static __is_nothrow_new_constructible_142: Value<bool> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __move_if_noexcept_cond_143 {}
impl Clone for __move_if_noexcept_cond_143 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __move_if_noexcept_cond_143 {
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
    pub static piecewise_construct_144: Value<std_piecewise_construct_t> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct tuple_145 {}
impl Clone for tuple_145 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for tuple_145 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl tuple_145 {}
impl tuple_145 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct initializer_list_146 {}
impl Clone for initializer_list_146 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for initializer_list_146 {
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
pub struct __are_same_147 {}
impl Clone for __are_same_147 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_147 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_148 = u32;
pub const anon_148___value: anon_148 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_149 {}
impl Clone for __are_same_149 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_149 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_150 = u32;
pub const anon_150___value: anon_150 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_151 {}
impl Clone for __are_same_151 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_151 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_152 = u32;
pub const anon_152___value: anon_152 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_153 {}
impl Clone for __are_same_153 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_153 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_154 = u32;
pub const anon_154___value: anon_154 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __are_same_155 {}
impl Clone for __are_same_155 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __are_same_155 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_156 = u32;
pub const anon_156___value: anon_156 = 0;
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
pub const anon_158___value: anon_158 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_159 {}
impl Clone for __is_integer_159 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_159 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_160 = u32;
pub const anon_160___value: anon_160 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_161 {}
impl Clone for __is_integer_161 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_161 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_162 = u32;
pub const anon_162___value: anon_162 = 1;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_163 {}
impl Clone for __is_integer_163 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_163 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_164 = u32;
pub const anon_164___value: anon_164 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_165 {}
impl Clone for __is_integer_165 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_165 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_166 = u32;
pub const anon_166___value: anon_166 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_167 {}
impl Clone for __is_integer_167 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_167 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_168 = u32;
pub const anon_168___value: anon_168 = 0;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __memcpyable_integer_169 {}
impl Clone for __memcpyable_integer_169 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __memcpyable_integer_169 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_170 {}
impl Clone for __is_integer_nonstrict_170 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_170 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_171 = u32;
pub const anon_171___width: anon_171 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_integer_nonstrict_172 {}
impl Clone for __is_integer_nonstrict_172 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_integer_nonstrict_172 {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type anon_173 = u32;
pub const anon_173___width: anon_173 = 64;
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_174 {}
impl Clone for __numeric_traits_integer_174 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_174 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____is_signed_175: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____digits_176: Value<i32> = unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_long____max_177: Value<i64> = unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_integer_178 {}
impl Clone for __numeric_traits_integer_178 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_integer_178 {
    fn default() -> Self {
        unimplemented!()
    }
}
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____is_signed_179: Value<bool> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____digits_180: Value<i32> =
        unimplemented!();
);
thread_local!(
    pub static __gnu_cxx___numeric_traits_integer_unsigned_long____max_181: Value<u64> =
        unimplemented!();
);
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_182 {}
impl Clone for __numeric_traits_floating_182 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_182 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __numeric_traits_floating_183 {}
impl Clone for __numeric_traits_floating_183 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_floating_183 {
    fn default() -> Self {
        unimplemented!()
    }
}
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
pub struct __numeric_traits_185 {}
impl Clone for __numeric_traits_185 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __numeric_traits_185 {
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
pub struct iterator_186 {}
impl Clone for iterator_186 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_186 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_187 {}
impl Clone for iterator_187 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_187 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl iterator_187 {}
impl iterator_187 {}
impl iterator_187 {}
impl iterator_187 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_187>) -> Ptr<iterator_187> {
        unimplemented!()
    }
}
impl iterator_187 {
    pub fn move_assign(&self, _a0: Ptr<iterator_187>) -> Ptr<iterator_187> {
        unimplemented!()
    }
}
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
impl iterator_188 {
    pub fn copy_assign(&self, _a0: Ptr<iterator_188>) -> Ptr<iterator_188> {
        unimplemented!()
    }
}
impl iterator_188 {
    pub fn move_assign(&self, _a0: Ptr<iterator_188>) -> Ptr<iterator_188> {
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
pub struct iterator_traits_190 {}
impl Clone for iterator_traits_190 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_190 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct iterator_traits_191 {}
impl Clone for iterator_traits_191 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for iterator_traits_191 {
    fn default() -> Self {
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
pub struct __iterator_traits_194 {}
impl Clone for __iterator_traits_194 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_194 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __iterator_traits_195 {}
impl Clone for __iterator_traits_195 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __iterator_traits_195 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __ptr_traits_ptr_to_196 {}
impl Clone for __ptr_traits_ptr_to_196 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __ptr_traits_ptr_to_196 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct pointer_traits_197 {}
impl Clone for pointer_traits_197 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for pointer_traits_197 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_198 {}
impl Clone for reverse_iterator_198 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_198 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_198 {
    pub fn copy_from(__x: Ptr<reverse_iterator_198>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_198 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_198>) -> Ptr<reverse_iterator_198> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(16)]
pub struct reverse_iterator_199 {}
impl Clone for reverse_iterator_199 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for reverse_iterator_199 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_199 {
    pub fn copy_from(__x: Ptr<reverse_iterator_199>) -> Self {
        unimplemented!()
    }
}
impl reverse_iterator_199 {
    pub fn copy_assign(&self, _a0: Ptr<reverse_iterator_199>) -> Ptr<reverse_iterator_199> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_200 {}
impl Clone for __normal_iterator_200 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_200 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_200 {
    pub fn new_1(__i: Ptr<Ptr<MoveOnly>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_200 {}
impl __normal_iterator_200 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct __normal_iterator_201 {}
impl Clone for __normal_iterator_201 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __normal_iterator_201 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_201 {
    pub fn new_1(__i: Ptr<Ptr<MoveOnly>>) -> Self {
        unimplemented!()
    }
}
impl __normal_iterator_201 {}
impl __normal_iterator_201 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(8)]
pub struct move_iterator_202 {}
impl Clone for move_iterator_202 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for move_iterator_202 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl move_iterator_202 {
    pub fn new_1(__i: Ptr<MoveOnly>) -> Self {
        unimplemented!()
    }
}
impl move_iterator_202 {}
impl move_iterator_202 {}
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
pub struct __new_allocator_203 {}
impl Clone for __new_allocator_203 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_203 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_203 {
    pub fn copy_from(_a0: Ptr<__new_allocator_203>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_203 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_203>) -> Ptr<__new_allocator_203> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_204 {}
impl Clone for __new_allocator_204 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_204 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_204 {
    pub fn copy_from(_a0: Ptr<__new_allocator_204>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_204 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_204>) -> Ptr<__new_allocator_204> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __new_allocator_205 {}
impl Clone for __new_allocator_205 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __new_allocator_205 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_205 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl __new_allocator_205 {
    pub fn copy_from(_a0: Ptr<__new_allocator_205>) -> Self {
        unimplemented!()
    }
}
impl __new_allocator_205 {
    pub fn copy_assign(&self, _a0: Ptr<__new_allocator_205>) -> Ptr<__new_allocator_205> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_207 {}
impl Clone for allocator_207 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_207 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_207 {}
impl allocator_207 {}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_208 {}
impl Clone for allocator_208 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_208 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_208 {
    pub fn new() -> Self {
        unimplemented!()
    }
}
impl allocator_208 {
    pub fn copy_from(__a: Ptr<allocator_208>) -> Self {
        unimplemented!()
    }
}
impl allocator_208 {
    pub fn copy_assign(&self, _a0: Ptr<allocator_208>) -> Ptr<allocator_208> {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct allocator_traits_209 {}
impl Clone for allocator_traits_209 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for allocator_traits_209 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl allocator_traits_209 {
    pub fn allocate_1(__a: Ptr<allocator_208>, mut __n: u64) -> Ptr<MoveOnly> {
        unimplemented!()
    }
}
impl allocator_traits_209 {
    pub fn deallocate(__a: Ptr<allocator_208>, mut __p: Ptr<MoveOnly>, mut __n: u64) {
        unimplemented!()
    }
}
impl allocator_traits_209 {
    pub fn max_size(__a: Ptr<allocator_208>) -> u64 {
        unimplemented!()
    }
}
impl allocator_traits_209 {
    pub fn construct_210(__a: Ptr<allocator_208>, mut __p: Ptr<MoveOnly>, __args: Ptr<MoveOnly>) {
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
pub struct __is_move_insertable_212 {}
impl Clone for __is_move_insertable_212 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_move_insertable_212 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __alloc_traits_213 {}
impl Clone for __alloc_traits_213 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __alloc_traits_213 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct rebind_214 {}
impl Clone for rebind_214 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for rebind_214 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(16)]
pub struct _UninitDestroyGuard_215 {
    #[offset(0)]
    #[byte_size(8)]
    pub _M_first: Ptr<MoveOnly>,
    #[offset(8)]
    #[byte_size(8)]
    pub _M_cur: Ptr<Ptr<MoveOnly>>,
}
impl Clone for _UninitDestroyGuard_215 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _UninitDestroyGuard_215 {
    fn default() -> Self {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_215 {
    pub fn new(__first: Ptr<Ptr<MoveOnly>>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __is_bitwise_relocatable_216 {}
impl Clone for __is_bitwise_relocatable_216 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __is_bitwise_relocatable_216 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Record, ByteRepr)]
#[byte_size(24)]
pub struct _Vector_base_217 {
    #[offset(0)]
    #[byte_size(24)]
    pub _M_impl: std__Vector_base_MoveOnly__std_allocator_MoveOnly____Vector_impl,
}
impl Clone for _Vector_base_217 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for _Vector_base_217 {
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
impl _Vector_base_217 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl _Vector_base_217 {
    pub fn move_from(_a0: Ptr<_Vector_base_217>) -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(24)]
pub struct vector_218 {}
impl Clone for vector_218 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for vector_218 {
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
    pub _M_vect: Ptr<_Vector_base_217>,
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
    pub fn new(mut __s: Ptr<MoveOnly>, mut __l: u64, __vect: Ptr<_Vector_base_217>) -> Self {
        unimplemented!()
    }
}
impl vector_218 {
    pub fn new_1() -> Self {
        unimplemented!()
    }
}
impl vector_218 {
    pub fn copy_from(__x: Ptr<vector_218>) -> Self {
        unimplemented!()
    }
}
impl vector_218 {
    pub fn move_from(_a0: Ptr<vector_218>) -> Self {
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
pub struct __hash_base_224 {}
impl Clone for __hash_base_224 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_224 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_225 {}
impl Clone for __hash_base_225 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_225 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_226 {}
impl Clone for __hash_base_226 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_226 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_227 {}
impl Clone for __hash_base_227 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_227 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_228 {}
impl Clone for __hash_base_228 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_228 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_229 {}
impl Clone for __hash_base_229 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_229 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_230 {}
impl Clone for __hash_base_230 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_230 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_231 {}
impl Clone for __hash_base_231 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_231 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_232 {}
impl Clone for __hash_base_232 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_232 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_233 {}
impl Clone for __hash_base_233 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_233 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_234 {}
impl Clone for __hash_base_234 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_234 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_235 {}
impl Clone for __hash_base_235 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_235 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_236 {}
impl Clone for __hash_base_236 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_236 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_237 {}
impl Clone for __hash_base_237 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_237 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_238 {}
impl Clone for __hash_base_238 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_238 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_239 {}
impl Clone for __hash_base_239 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_239 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct __hash_base_240 {}
impl Clone for __hash_base_240 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for __hash_base_240 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct hash_241 {}
impl Clone for hash_241 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for hash_241 {
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
    pub fn hash_242(__val: Ptr<f32>) -> usize {
        unimplemented!()
    }
}
impl std__Hash_impl {
    pub fn hash_243(__val: Ptr<f64>) -> usize {
        unimplemented!()
    }
}
pub type anon_244 = u32;
pub const anon_244__S_word_bit: anon_244 = 64;
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
pub struct binary_function_245 {}
impl Clone for binary_function_245 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for binary_function_245 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_246 {}
impl Clone for greater_246 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_246 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_247 {}
impl Clone for less_247 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_247 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct greater_equal_248 {}
impl Clone for greater_equal_248 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for greater_equal_248 {
    fn default() -> Self {
        unimplemented!()
    }
}
#[derive(VaArg, FnPtrArg, Copy, Record, ByteRepr)]
#[byte_size(1)]
pub struct less_equal_249 {}
impl Clone for less_equal_249 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for less_equal_249 {
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
    pub static allocator_arg_250: Value<std_allocator_arg_t> = unimplemented!();
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
pub struct ConstMoveAssign {
    #[offset(0)]
    pub mark: i32,
}
impl ConstMoveAssign {
    pub fn new() -> Self {
        Self { mark: 0 }
    }
}
impl Default for ConstMoveAssign {
    fn default() -> Self {
        { ConstMoveAssign::new() }
    }
}
pub fn make_251(mut v: i32) -> MoveOnly {
    let m: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ v })));
    return MoveOnly::move_from({ m.as_pointer() });
}
pub fn main() {
    __cpp2rust_init_globals();
    std::process::exit(main_0());
}
fn main_0() -> i32 {
    let a: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 1 })));
    let b: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 2 })));
    let c: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 3 })));
    ({ MoveOnlyImpl::move_assign(&a.as_pointer(), b.as_pointer()) });
    (if ({ (*a.borrow()).v } == 2) {
        &(0);
    } else {
        ({
            __assert_fail_252(
                Ptr::<i8>::from_string_literal(b"a.v == 2"),
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
            __assert_fail_252(
                Ptr::<i8>::from_string_literal(b"b.v == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _o: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 3 })));
        MoveOnlyImpl::move_assign(&b.as_pointer(), _o.as_pointer())
    });
    ({
        MoveOnlyImpl::move_assign(
            &c.as_pointer(),
            ({ MoveOnlyImpl::move_assign(&a.as_pointer(), b.as_pointer()) }),
        )
    });
    (if (({ (*b.borrow()).v } == 0) && ({ (*a.borrow()).v } == 0)) && ({ (*c.borrow()).v } == 3) {
        &(0);
    } else {
        ({
            __assert_fail_252(
                Ptr::<i8>::from_string_literal(b"b.v == 0 && a.v == 0 && c.v == 3"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _o: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 5 })));
        MoveOnlyImpl::move_assign(&a.as_pointer(), _o.as_pointer())
    });
    (if ({ (*a.borrow()).v } == 5) {
        &(0);
    } else {
        ({
            __assert_fail_252(
                Ptr::<i8>::from_string_literal(b"a.v == 5"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _o: Value<MoveOnly> = Rc::new(RefCell::new(({ make_251(6) })));
        MoveOnlyImpl::move_assign(&a.as_pointer(), _o.as_pointer())
    });
    (if ({ (*a.borrow()).v } == 6) {
        &(0);
    } else {
        ({
            __assert_fail_252(
                Ptr::<i8>::from_string_literal(b"a.v == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    ({
        let _o: Ptr<MoveOnly> = a.as_pointer();
        MoveOnlyImpl::move_assign(&a.as_pointer(), _o)
    });
    (if ({ (*a.borrow()).v } == 6) {
        &(0);
    } else {
        ({
            __assert_fail_252(
                Ptr::<i8>::from_string_literal(b"a.v == 6"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let vec_: Value<vector_218> = Rc::new(RefCell::new(<vector_218>::default()));
    ({
        let ___x: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 7 })));
        vector_218Impl::push_back_58(&vec_.as_pointer(), ___x.as_pointer())
    });
    let d: Value<MoveOnly> = Rc::new(RefCell::new(MoveOnly::new({ 8 })));
    ({
        MoveOnlyImpl::move_assign(
            &({ vector_218Impl::operator_index_46(&vec_.as_pointer(), 0_u64) }),
            d.as_pointer(),
        )
    });
    (if (({ vector_218Impl::operator_index_46(&vec_.as_pointer(), 0_u64) }).with(|__s| __s.v) == 8)
    {
        &(0);
    } else {
        ({
            __assert_fail_252(
                Ptr::<i8>::from_string_literal(b"vec[0].v == 8"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*d.borrow()).v } == 0) {
        &(0);
    } else {
        ({
            __assert_fail_252(
                Ptr::<i8>::from_string_literal(b"d.v == 0"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    let m: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::new()));
    let m1: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::new()));
    let m2: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::new()));
    let cm: Value<ConstMoveAssign> = Rc::new(RefCell::new(ConstMoveAssign::new()));
    ({ ConstMoveAssignImpl::operator_assign_2(&m1.as_pointer(), m.as_pointer()) });
    ({ ConstMoveAssignImpl::operator_assign_3(&m2.as_pointer(), cm.as_pointer()) });
    (if ({ (*m1.borrow()).mark } == 1) {
        &(0);
    } else {
        ({
            __assert_fail_252(
                Ptr::<i8>::from_string_literal(b"m1.mark == 1"),
                Ptr::<i8>::from_string_literal(file!().as_bytes()),
                line!() as u32,
                Ptr::<i8>::from_string_literal(b"int main()"),
            )
        });
    });
    (if ({ (*m2.borrow()).mark } == 10) {
        &(0);
    } else {
        ({
            __assert_fail_252(
                Ptr::<i8>::from_string_literal(b"m2.mark == 10"),
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
pub fn __assert_fail_252(
    mut __assertion: Ptr<i8>,
    mut __file: Ptr<i8>,
    mut __line: u32,
    mut __function: Ptr<i8>,
) {
    unimplemented!()
}
pub trait ConstMoveAssignImpl {
    fn operator_assign_2(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign>;
    fn operator_assign_3(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign>;
}
impl ConstMoveAssignImpl for Ptr<ConstMoveAssign> {
    fn operator_assign_2(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 1) });
        return (*self).clone();
    }
    fn operator_assign_3(&self, o: Ptr<ConstMoveAssign>) -> Ptr<ConstMoveAssign> {
        field!((*self), mark).write({ (o.with(|__s| __s.mark) + 10) });
        return (*self).clone();
    }
}
pub trait MoveOnlyImpl {
    fn move_assign(&self, o: Ptr<MoveOnly>) -> Ptr<MoveOnly>;
}
impl MoveOnlyImpl for Ptr<MoveOnly> {
    fn move_assign(&self, o: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        if ((*self) == (o)) {
            return (*self).clone();
        }
        field!((*self), v).write({ o.with(|__s| __s.v) });
        field!(o, v).write(0);
        return (*self).clone();
    }
}
pub trait _UninitDestroyGuard_215Impl {
    fn release(&self) {
        unimplemented!()
    }
}
impl _UninitDestroyGuard_215Impl for Ptr<_UninitDestroyGuard_215> {}
pub trait _Vector_base_217Impl {
    fn _M_get_Tp_allocator_1(&self) -> Ptr<allocator_208> {
        unimplemented!()
    }
    fn _M_get_Tp_allocator_2(&self) -> Ptr<allocator_208> {
        unimplemented!()
    }
    fn _M_allocate(&self, mut __n: usize) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn _M_deallocate(&self, mut __p: Ptr<MoveOnly>, mut __n: usize) {
        unimplemented!()
    }
}
impl _Vector_base_217Impl for Ptr<_Vector_base_217> {}
pub trait __new_allocator_205Impl {
    fn allocate(&self, mut __n: u64, _a1: Option<AnyPtr>) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn deallocate(&self, mut __p: Ptr<MoveOnly>, mut __n: u64) {
        unimplemented!()
    }
    fn max_size(&self) -> u64 {
        unimplemented!()
    }
    fn construct_206(&self, mut __p: Ptr<MoveOnly>, __args: Ptr<MoveOnly>) {
        unimplemented!()
    }
}
impl __new_allocator_205Impl for Ptr<__new_allocator_205> {}
pub trait __normal_iterator_200Impl {
    fn operator_deref(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_sub(&self, mut __n: i64) -> __normal_iterator_200 {
        unimplemented!()
    }
    fn base(&self) -> Ptr<Ptr<MoveOnly>> {
        unimplemented!()
    }
}
impl __normal_iterator_200Impl for Ptr<__normal_iterator_200> {}
pub trait __normal_iterator_201Impl {
    fn base(&self) -> Ptr<Ptr<MoveOnly>> {
        unimplemented!()
    }
}
impl __normal_iterator_201Impl for Ptr<__normal_iterator_201> {}
pub trait initializer_list_146Impl {
    fn size(&self) -> u64 {
        unimplemented!()
    }
    fn begin(&self) -> Ptr<bool> {
        unimplemented!()
    }
}
impl initializer_list_146Impl for Ptr<initializer_list_146> {}
pub trait integral_constant_0Impl {
    fn to_bool(&self) -> bool {
        unimplemented!()
    }
}
impl integral_constant_0Impl for Ptr<integral_constant_0> {}
pub trait move_iterator_202Impl {
    fn base(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_deref(&self) -> Ptr<MoveOnly> {
        unimplemented!()
    }
    fn operator_inc_8(&self) -> Ptr<move_iterator_202> {
        unimplemented!()
    }
}
impl move_iterator_202Impl for Ptr<move_iterator_202> {}
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
pub trait vector_218Impl {
    fn copy_assign(&self, __x: Ptr<vector_218>) -> Ptr<vector_218> {
        unimplemented!()
    }
    fn move_assign(&self, __x: Ptr<vector_218>) -> Ptr<vector_218> {
        unimplemented!()
    }
    fn begin_26(&self) -> __normal_iterator_200 {
        unimplemented!()
    }
    fn begin_27(&self) -> __normal_iterator_201 {
        unimplemented!()
    }
    fn end_28(&self) -> __normal_iterator_200 {
        unimplemented!()
    }
    fn end_29(&self) -> __normal_iterator_201 {
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
    fn emplace_back_219(&self, __args: Ptr<MoveOnly>) -> Ptr<MoveOnly> {
        unimplemented!()
    }
}
impl vector_218Impl for Ptr<vector_218> {}
pub fn __cpp2rust_init_globals() {}
